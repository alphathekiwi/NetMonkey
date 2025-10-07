use std::fs::read_to_string;
use std::net::IpAddr;

use crate::Msg;
use crate::adaptor::NetworkAdapter;

use crate::components::{LabelWithHint, SubnetSlider, TextInputDropdown};
use crate::theme::NetMonkeyTheme;
use crate::views::ip_scan::ScannedIp;
use iced::Alignment::Center;
use iced::Element;
use iced::widget::{column, scrollable, text, text_input};
use iced_widget::{horizontal_rule, pick_list};
use serde::{Deserialize, Serialize};

pub fn view<'a>(app: &'a IpScannerApp) -> Element<'a, Msg> {
    let items = app
        .adaptors
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>();
    println!("{items:?}");
    let ip_sel: TextInputDropdown<_, _, Msg, iced::Theme> = TextInputDropdown::new(
        items,
        app.config.starting_ip.to_string(),
        |s| Msg::Config(ChangeConfig::StartingIp(s)),
        |s| Msg::Config(ChangeConfig::StartingIp(s)),
    )
    .text_size(24);
    let subnet_slider = SubnetSlider::new(app.config.subnet_mask, Msg::subnet_mask)
        .text_size(24.0)
        .height(45.0)
        .into_element();

    // Create theme selector dropdown
    let theme_options = [
        NetMonkeyTheme::Dark,
        NetMonkeyTheme::Light,
        NetMonkeyTheme::HighContrast,
    ];
    let theme_selector = pick_list(theme_options, Some(app.config.theme), |theme| {
        Msg::Config(ChangeConfig::Theme(theme))
    })
    .text_size(24);

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
            .theme(app.config.theme)
            .into_element(),
            text_input("Ports List", &app.config.ports_to_string())
                .on_input(|s| Msg::Config(ChangeConfig::Ports(s)))
                .size(24),
            text("Appearance").size(22),
            horizontal_rule(2),
            LabelWithHint::new("Theme", app.config.theme.description())
                .text_size(18.0)
                .theme(app.config.theme)
                .into_element(),
            iced::Element::from(theme_selector),
        ]
        .align_x(Center)
        .spacing(12)
        .padding(20),
    )
    .height(iced::Length::Fill)
    .direction(iced::widget::scrollable::Direction::Vertical(
        iced::widget::scrollable::Scrollbar::new()
            .width(8)
            .scroller_width(8),
    ))
    .into()
}

#[derive(Debug)]
pub struct IpScannerApp {
    pub tab: ModeTab,
    // IP Scanner
    pub ips: Vec<ScannedIp>,
    pub scan_progress: u8,
    // TCP Connection
    pub tcp_ip_port: String,
    pub tcp_ip_address: String,
    pub tcp_connection: Option<IpAddr>,
    pub tcp_history: Vec<String>,
    // UDP Connection
    pub udp_ip_port: String,
    pub udp_ip_address: String,
    pub udp_connection: Option<IpAddr>,
    pub udp_history: Vec<String>,
    // Settings
    pub adaptors: Vec<NetworkAdapter>,
    pub config: AppConfig,
}
impl IpScannerApp {
    pub fn loaded(&mut self, c: AppConfig, a: Vec<NetworkAdapter>) {
        self.config = c;
        self.adaptors = a;
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
    pub theme: NetMonkeyTheme,
}
impl Default for AppConfig {
    fn default() -> Self {
        Self {
            starting_ip: String::from("192.168.1.1"),
            subnet_mask: 24,
            ports: vec![80, 443],
            forced_ip_mode: ForcedIPMode::Any,
            theme: NetMonkeyTheme::Dark,
        }
    }
}
impl AppConfig {
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
            ChangeConfig::Theme(theme) => self.theme = theme,
        }
    }
    pub fn load() -> Option<Self> {
        serde_json::from_str(&read_to_string("data/config.json").ok()?).ok()
    }
    pub fn save(&self) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write("data/config.json", json)?;
        Ok(())
    }
}
impl Drop for AppConfig {
    fn drop(&mut self) {
        if let Err(e) = self.save() {
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModeTab {
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

/// Default implementation for IpScannerApp
impl Default for IpScannerApp {
    fn default() -> Self {
        let config = AppConfig::default();
        Self {
            tab: ModeTab::IpScan,
            scan_progress: 255,
            ips: Vec::new(),
            adaptors: vec![NetworkAdapter::default()],
            config,

            tcp_ip_port: String::new(),
            tcp_ip_address: String::new(),
            tcp_connection: None,
            tcp_history: Vec::new(),

            udp_connection: None,
            udp_history: Vec::new(),
            udp_ip_port: String::new(),
            udp_ip_address: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ChangeConfig {
    StartingIp(String),
    SubnetMask(String),
    Ports(String),
    ForcedIPMode(usize),
    Theme(NetMonkeyTheme),
}
