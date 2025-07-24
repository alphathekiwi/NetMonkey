use std::net::IpAddr;

use iced::Alignment::Center;
use iced::widget::{Column, column, text, text_input};

use crate::Msg;
use crate::views::ip_scan::ScannedIp;

pub fn view<'a>() -> Column<'a, Msg> {
    column![
        text("Settings").size(24),
        text("Starting IP").size(16),
        text_input("Starting IP", "").size(24),
        text("Subnet Mask").size(16),
        text_input("Subnet Mask", "").size(24),
        text("Ports List").size(16),
        text_input("Ports List", "").size(24),
    ]
    .align_x(Center)
    .spacing(10)
}

#[allow(unused)]
#[derive(Debug)]
pub struct IpScannerApp {
    pub tab: ModeTab,
    pub forced_ip_mode: ForcedIPMode,
    pub adaptors: Vec<IpAddr>,
    pub starting_ip: IpAddr,
    pub subnet_mask: u8,
    pub ports: Vec<u16>,
    pub ips: Vec<ScannedIp>,
    pub scan_progress: u8,
}
impl Default for IpScannerApp {
    fn default() -> Self {
        Self {
            tab: ModeTab::IpScan,
            forced_ip_mode: ForcedIPMode::Any,
            starting_ip: IpAddr::V4(std::net::Ipv4Addr::new(192, 168, 1, 1)),
            subnet_mask: 24,
            ports: vec![80, 443],
            scan_progress: 0,
            ips: Vec::new(),
            adaptors: Vec::new(),
        }
    }
}
#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForcedIPMode {
    Any,
    V4,
    V6,
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
