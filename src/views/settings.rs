use std::fs::read_to_string;
use std::net::IpAddr;

use crate::Msg;
use crate::adaptor::{NetworkAdapter, get_network_adapters};
use crate::views::ip_scan::ScannedIp;
use iced::Alignment::Center;
use iced::Length::Fixed;
use iced::widget::text::LineHeight::Absolute;
use iced::widget::{Column, column, combo_box, pick_list, row, text, text_input, vertical_slider};
use serde::{Deserialize, Serialize};

pub fn view<'a>(app: &'a IpScannerApp) -> Column<'a, Msg> {
    let none = None::<&NetworkAdapter>;
    column![
        text("Settings").size(24),
        text("Starting IP").size(16),
        row![
            text_input("Starting IP", &app.config.starting_ip.to_string())
                .on_input(|s| Msg::Config(ChangeConfig::StartingIp(s)))
                .size(24),
            // button("▼").on_press(Msg::Adaptors)
            pick_list(app.adaptors.clone(), none, Msg::Adaptor)
                .width(Fixed(32.0))
                .text_line_height(Absolute(32.into())),
            // combo_box(&app.dropdown, "Select Adapter", none, Msg::Adaptor) // .width(Fixed(32.0))
        ],
        text("Subnet Mask").size(16),
        row![
            vertical_slider(1..=32, app.config.subnet_mask, Msg::subnet_mask),
            // .on_input(|s| Msg::Config(ChangeConfig::SubnetMask(s)))
            // .size(24)
            text_input("Subnet Mask", &app.config.subnet_mask_long()).size(24),
        ],
        text("Ports List").size(16),
        row![
            text_input("Ports List", &app.config.ports_to_string())
                .on_input(|s| Msg::Config(ChangeConfig::Ports(s)))
                .size(24),
        ],
    ]
    .align_x(Center)
    .spacing(10)
}

#[derive(Debug)]
pub struct IpScannerApp {
    pub tab: ModeTab,
    pub scan_progress: u8,

    pub adaptors: Vec<NetworkAdapter>,
    pub ips: Vec<ScannedIp>,
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
