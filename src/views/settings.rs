use std::fs::read_to_string;
use std::net::IpAddr;

use crate::Msg;
use crate::adaptor::{NetworkAdapter, get_network_adapters};
use crate::components::adaptors::TextInputDropdown;
use crate::views::ip_scan::ScannedIp;
use iced::Alignment::Center;
use iced::widget::{Column, column, combo_box, row, text, text_input, vertical_slider};
use iced_core::Widget;
use serde::{Deserialize, Serialize};

pub fn view<'a>(app: &'a IpScannerApp) -> Column<'a, Msg> {
    let items = app
        .adaptors
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>();
    let ip_sel: TextInputDropdown<String, Vec<String>, Msg, iced::Theme> = TextInputDropdown::new(
        items,
        app.config.starting_ip.to_string(),
        |s| Msg::Config(ChangeConfig::StartingIp(s)),
        |s| Msg::Config(ChangeConfig::StartingIp(s)),
    )
    .size(24.into());
    column![
        text("Settings").size(24),
        text("Starting IP").size(16),
        iced::Element::from(ip_sel),
        text("Subnet Mask").size(16),
        row![
            vertical_slider(1..=32, app.config.subnet_mask, Msg::subnet_mask),
            // .on_input(|s| Msg::Config(ChangeConfig::SubnetMask(s)))
            // .size(24)
            text_input("Subnet Mask", &app.config.subnet_mask_long()).size(24),
        ],
        text("Ports List").size(16),
        text_input("Ports List", &app.config.ports_to_string())
            .on_input(|s| Msg::Config(ChangeConfig::Ports(s)))
            .size(24),
    ]
    .align_x(Center)
    .spacing(10)
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
    pub dropdown: combo_box::State<NetworkAdapter>,
}
impl IpScannerApp {
    pub fn loaded(&mut self, c: AppConfig, a: Vec<NetworkAdapter>) {
        self.config = c;
        self.dropdown = combo_box::State::new(a.clone());
        self.adaptors = a;
    }
}

// let state = SettingsState {
//     state: combo_box::State::new(adaptors.into()),
//     selected: None,
// };
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppConfig {
    pub starting_ip: IpAddr,
    pub subnet_mask: u8,
    pub ports: Vec<u16>,
    pub forced_ip_mode: ForcedIPMode,
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
            ChangeConfig::StartingIp(ip) => self.starting_ip = ip.parse().unwrap(),
            ChangeConfig::SubnetMask(mask) => self.subnet_mask = mask.parse().unwrap(),
            ChangeConfig::Ports(ports) => {
                self.ports = ports.split(',').map(|p| p.parse().unwrap()).collect()
            }
            ChangeConfig::ForcedIPMode(mode) => self.forced_ip_mode = mode.into(),
        }
    }
    pub async fn load() -> Option<(Self, Vec<NetworkAdapter>)> {
        serde_json::from_str(&read_to_string("data/config.json").ok()?)
            .ok()
            .map(|v| (v, get_network_adapters()))
        // .map(|v| (v, Vec::new()))
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
        let config = AppConfig {
            starting_ip: IpAddr::V4(std::net::Ipv4Addr::new(192, 168, 1, 1)),
            subnet_mask: 24,
            ports: vec![80, 443],
            forced_ip_mode: ForcedIPMode::Any,
        };
        Self {
            tab: ModeTab::IpScan,
            scan_progress: 255,
            ips: Vec::new(),
            adaptors: Vec::new(),
            dropdown: combo_box::State::new(Vec::new()),
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
}
