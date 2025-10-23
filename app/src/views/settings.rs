use std::fs::read_to_string;
use std::net::IpAddr;

use crate::Msg;

use iced::Alignment::Center;
use iced::Element;
use iced::Length::Fill;
use iced::widget::{column, scrollable, text, text_input};
use iced_widget::{horizontal_rule, row};
use net_monkey_components::{LabelWithHint, SubnetSlider, TextInputDropdown};
use net_monkey_core::{NetworkAdapter, ScannedIp};
use net_monkey_theme::ThemeProvider;
use serde::{Deserialize, Serialize};

pub fn view<'a>(app: &'a IpScannerApp) -> Element<'a, Msg> {
    let items = app.adaptors.clone();
    println!("{items:?}");
    let ip_sel: TextInputDropdown<_, _, Msg, iced::Theme> = TextInputDropdown::new(
        items,
        app.config.starting_ip.to_string(),
        |s| Msg::Config(ChangeConfig::StartingIp(s)),
        |s| Msg::Config(ChangeConfig::StartingIp(s.ip_address)),
    )
    .text_size(24);
    let subnet_slider = SubnetSlider::new(app.config.subnet_mask, Msg::subnet_mask)
        .text_size(24.0)
        .height(45.0)
        .into_element();

    scrollable(
        column![
            text("Network Configuration").size(22),
            horizontal_rule(2),
            text("Starting IP").size(18),
            iced::Element::from(ip_sel),
            text("Subnet Mask").size(18),
            subnet_slider,
            LabelWithHint::new(
                "Ports List",
                "Comma-separated list of ports to scan (e.g., 80, 443, 22)"
            )
            .text_size(18.0)
            .theme(app.config.theme_provider())
            .into_element(),
            text_input("Ports List", &app.config.ports_to_string())
                .on_input(|s| Msg::Config(ChangeConfig::Ports(s)))
                .size(24),
            text("Appearance").size(22),
            horizontal_rule(2),
            LabelWithHint::new("Theme", app.config.theme_provider().name())
                .text_size(18.0)
                .theme(app.config.theme_provider())
                .into_element(),
            row![text("COSMIC Theme (System-managed)").size(24).width(Fill),].spacing(8),
        ]
        .align_x(Center)
        .spacing(12)
        .padding(20),
    )
    .height(Fill)
    .direction(iced::widget::scrollable::Direction::Vertical(
        iced::widget::scrollable::Scrollbar::new()
            .width(8)
            .scroller_width(8),
    ))
    .into()
}

#[derive(Debug, Default)]
pub struct ConnectionData {
    pub ip_port: String,
    pub ip_address: String,
    pub current_packet: String,
    pub connections: Vec<IpAddr>,
    pub history: Vec<String>,
}
impl ConnectionData {
    pub fn update(&mut self, msg: Msg) {
        match msg {
            Msg::ChangePacket(pak) => self.current_packet = pak,
            Msg::ChangeIpAddress(ip) => self.ip_address = ip,
            Msg::ChangeIpPort(port) => self.ip_port = port,
            Msg::SendPacket => self.history.push(self.current_packet.clone()),
            Msg::ConnectionToggle if self.connections.is_empty() => {
                if let Ok(conn) = IpAddr::parse_ascii(self.ip_address.as_bytes()) {
                    self.connections.push(conn)
                }
            }
            Msg::ConnectionToggle => self.connections.clear(),
            _ => {}
        }
    }
}
#[derive(Debug, Default)]
pub struct IpScannerApp {
    #[cfg(feature = "cosmic")]
    pub core: cosmic::app::Core,
    pub tab: ModeTab,
    // IP Scanner
    pub ips: Vec<ScannedIp>,
    pub scan_progress: u8,
    pub loaded: bool,
    pub tcp_client: ConnectionData,
    pub udp_client: ConnectionData,
    pub tcp_server: ConnectionData,
    pub udp_server: ConnectionData,
    // Settings
    pub adaptors: Vec<NetworkAdapter>,
    pub config: AppConfig,
}

impl IpScannerApp {
    pub fn loaded(&mut self, c: AppConfig, a: Vec<NetworkAdapter>) {
        self.config = c;
        self.adaptors = a;
        self.loaded = true;
    }
}

// let state = SettingsState {
//     state: combo_box::State::new(adaptors.into()),
//     selected: None,
// };
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppConfig {
    pub starting_ip: String,
    pub subnet_mask: u8,
    pub ports: Vec<u16>,
    pub forced_ip_mode: ForcedIPMode,
}
impl Default for AppConfig {
    fn default() -> Self {
        Self {
            starting_ip: String::from("192.168.1.1"),
            subnet_mask: 24,
            ports: vec![80, 443],
            forced_ip_mode: ForcedIPMode::Any,
        }
    }
}
impl AppConfig {
    /// Get theme provider for this config
    pub fn theme_provider(&self) -> ThemeProvider {
        ThemeProvider::default()
    }
    pub fn ports_to_string(&self) -> String {
        self.ports
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(", ")
    }
    fn left_shift(&self, value: u8) -> u8 {
        match self.subnet_mask < value {
            true if value - self.subnet_mask > 7 => 0,
            true => 255 << (value - self.subnet_mask).min(8),
            false => 255,
        }
    }
    pub fn subnet_mask_long(&self) -> String {
        format!(
            "{}.{}.{}.{}",
            self.left_shift(8u8),
            self.left_shift(16u8),
            self.left_shift(24u8),
            self.left_shift(32u8)
        )
    }
    pub fn update(&mut self, change: ChangeConfig) {
        match change {
            ChangeConfig::StartingIp(ip) => self.starting_ip = ip,
            ChangeConfig::SubnetMask(mask) => self.subnet_mask = mask.parse().unwrap_or_default(),
            ChangeConfig::Ports(ports) => {
                self.ports = ports.split(',').filter_map(|p| p.parse().ok()).collect()
            }
            ChangeConfig::ForcedIPMode(mode) => self.forced_ip_mode = mode.into(),
        }
    }
    pub fn load() -> Option<Self> {
        let config_path = Self::config_file_path();
        serde_json::from_str(&read_to_string(config_path).ok()?).ok()
    }
    pub fn save(&self) -> anyhow::Result<()> {
        let config_path = Self::config_file_path();
        let config_dir = std::path::Path::new(&config_path).parent().unwrap();
        std::fs::create_dir_all(config_dir)?;
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(config_path, json)?;
        Ok(())
    }

    /// Get the config file path based on build mode
    fn config_file_path() -> String {
        #[cfg(debug_assertions)]
        {
            // In debug mode, find the workspace root and use app/data/config.json
            if let Ok(current_dir) = std::env::current_dir() {
                let mut path = current_dir;
                // Look for workspace Cargo.toml (contains [workspace]) to identify workspace root
                loop {
                    let cargo_toml = path.join("Cargo.toml");
                    if cargo_toml.exists()
                        && let Ok(content) = std::fs::read_to_string(&cargo_toml)
                        && content.contains("[workspace]")
                    {
                        break;
                    }
                    if !path.pop() {
                        // Fallback if we can't find workspace root
                        return "app/data/config.json".to_string();
                    }
                }
                path.push("app");
                path.push("data");
                path.push("config.json");
                path.to_string_lossy().to_string()
            } else {
                "app/data/config.json".to_string()
            }
        }
        #[cfg(not(debug_assertions))]
        {
            // In release mode, use current working directory
            "data/config.json".to_string()
        }
    }
}
// Implementation on App to prevent config being overwritten on load
impl Drop for IpScannerApp {
    fn drop(&mut self) {
        if let Err(e) = self.config.save() {
            eprintln!("Failed to save config: {e}");
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum ForcedIPMode {
    Any,
    V4,
    V6,
}
impl From<usize> for ForcedIPMode {
    fn from(mode: usize) -> Self {
        match mode {
            1 => ForcedIPMode::V4,
            2 => ForcedIPMode::V6,
            _ => ForcedIPMode::Any,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ModeTab {
    #[default]
    IpScan,
    TCPclient,
    TCPserver,
    UDPclient,
    UDPserver,
    Settings,
}

/// Basic to string conversion for ModeTab
impl From<&ModeTab> for String {
    fn from(tab: &ModeTab) -> Self {
        match tab {
            ModeTab::IpScan => "IP Scan",
            ModeTab::TCPclient => "TCP Client",
            ModeTab::TCPserver => "TCP Server",
            ModeTab::UDPclient => "UDP Client",
            ModeTab::UDPserver => "UDP Server",
            ModeTab::Settings => "Settings",
        }
        .to_string()
    }
}

#[derive(Debug, Clone)]
pub enum ChangeConfig {
    StartingIp(String),
    SubnetMask(String),
    Ports(String),
    ForcedIPMode(usize),
}

// Helper function to parse hex color
