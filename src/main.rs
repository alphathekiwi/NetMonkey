// #![allow(unused_imports, unused)]
#![feature(addr_parse_ascii)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::net::IpAddr;

use iced::keyboard::{Key, Modifiers, key::Named};
use iced::widget::image::Handle;
use iced::widget::{Image, Row, button, center, column, text};
use iced::widget::{button::Status, image as iced_image};
use iced::window::{Mode, Settings, icon::from_file_data};
use iced::{Center, Element, Fill, Subscription, Task as Command, Theme, keyboard, window};
use image::ImageFormat;

use crate::adaptor::NetworkAdapter;
use crate::adaptor::get_network_adapters;
use crate::views::ip_scan::ScannedIp;
use crate::views::settings::{AppConfig, ChangeConfig, IpScannerApp, ModeTab};

mod adaptor;
mod components;
mod views;

pub fn main() -> iced::Result {
    // #[cfg(not(target_arch = "wasm32"))]
    // tracing_subscriber::fmt::init();

    let window = Settings {
        icon: from_file_data(APP_ICON, Some(ImageFormat::Ico)).ok(),
        ..Default::default()
    };
    IpScannerApp::run_with(window)
}

const ICON_FONT: &[u8] = include_bytes!("../assets/icons.ttf");
const APP_BACKGROUND: &[u8] = include_bytes!("../assets/background.png");
const APP_ICON: &[u8] = include_bytes!("../assets/net_monkey.ico");
const TABS: &[ModeTab] = &[
    ModeTab::IpScan,
    ModeTab::TCPclient,
    ModeTab::UDPclient,
    ModeTab::Settings,
];
pub fn hero_image() -> Image<Handle> {
    let handle = Handle::from_bytes(APP_BACKGROUND);
    iced_image(handle).width(Fill).height(Fill)
}

#[derive(Debug, Clone)]
pub enum Msg {
    Loaded((AppConfig, Vec<NetworkAdapter>)),
    TabChanged(ModeTab),
    FocusMove { shift: bool },
    WinSize(Mode),
    BeginScan,
    ScanComplete,
    PingResult(ScannedIp),
    Testing,
    Config(ChangeConfig),
    Adaptor(NetworkAdapter),
    TcpIpPort(String),
    TcpIpAddress(String),
    TcpConnectionToggle,
    UdpIpPort(String),
    UdpIpAddress(String),
    UdpConnectionToggle,
}
impl Msg {
    fn key_press(any_key: Key, mods: Modifiers) -> Option<Msg> {
        let Key::Named(key) = any_key else {
            return None;
        };
        match (key, mods) {
            (Named::ArrowUp, Modifiers::SHIFT) => Some(Msg::WinSize(Mode::Fullscreen)),
            (Named::ArrowDown, Modifiers::SHIFT) => Some(Msg::WinSize(Mode::Windowed)),
            (Named::Tab, _) => Some(Msg::tab(mods.shift())),
            _ => None,
        }
    }
    fn tab(shift: bool) -> Self {
        Self::FocusMove { shift }
    }
    fn subnet_mask(subnet_mask: u8) -> Self {
        Self::Config(ChangeConfig::SubnetMask(subnet_mask.to_string()))
    }
}

impl IpScannerApp {
    fn run_with(window: Settings) -> Result<(), iced::Error> {
        iced::application("Net Monkey", Self::update, Self::view)
            .window_size((500.0, 800.0))
            .font(ICON_FONT)
            .window(window)
            .subscription(Self::subscription)
            .run_with(Self::initialize)
    }

    fn initialize() -> (Self, Command<Msg>) {
        (
            Self::default(),
            Command::perform(
                async {
                    (
                        AppConfig::load().unwrap_or_default(),
                        get_network_adapters(),
                    )
                },
                Msg::Loaded,
            ),
        )
    }

    fn update(&mut self, msg: Msg) -> Command<Msg> {
        use iced::widget::{focus_next, focus_previous};
        use window::{change_mode, get_latest};
        // All Msgs that return a Command
        let cmd = match &msg {
            Msg::WinSize(mode) => {
                let mode = *mode; // Copy the mode value
                get_latest().and_then(move |id| change_mode(id, mode))
            }
            Msg::FocusMove { shift: true } => focus_previous(),
            Msg::FocusMove { shift: false } => focus_next(),
            _ => Command::none(),
        };
        // All Msgs that should print
        match &msg {
            Msg::BeginScan => println!("Starting scan..."),
            Msg::ScanComplete => println!("Scan completed!"),
            Msg::Testing => println!("Test clicked"),
            Msg::Config(change) => println!("Updating config {change:?}"),
            _ => {}
        }
        // All Msgs that should update the state
        match msg {
            Msg::Loaded((c, a)) => self.loaded(c, a),
            Msg::PingResult(res) => {
                self.scan_progress += 1;
                self.ips.push(res);
            }
            Msg::TcpIpAddress(ip) => self.tcp_ip_address = ip,
            Msg::TcpIpPort(port) => self.tcp_ip_port = port,
            Msg::TcpConnectionToggle if self.tcp_connection.is_none() => {
                self.tcp_connection = IpAddr::parse_ascii(self.tcp_ip_address.as_bytes()).ok()
            }
            Msg::TcpConnectionToggle => self.tcp_connection = None,
            Msg::UdpIpAddress(ip) => self.udp_ip_address = ip,
            Msg::UdpIpPort(port) => self.udp_ip_port = port,
            Msg::UdpConnectionToggle if self.udp_connection.is_none() => {
                self.udp_connection = IpAddr::parse_ascii(self.udp_ip_address.as_bytes()).ok()
            }
            Msg::UdpConnectionToggle => self.udp_connection = None,
            Msg::TabChanged(tab) => self.tab = tab,
            Msg::BeginScan => self.scan_progress = 0,
            Msg::ScanComplete => self.scan_progress = 255,
            Msg::Config(change) => self.config.update(change),
            Msg::Adaptor(a) => self.config.update(ChangeConfig::StartingIp(a.ip_address)),
            _ => {}
        }
        cmd
    }

    fn view(&self) -> Element<'_, Msg> {
        let tabs = self.render_tabs();
        let col = match self.tab {
            ModeTab::IpScan => views::ip_scan::view(self),
            ModeTab::TCPclient | ModeTab::TCPserver => views::tcp_client::view(self),
            ModeTab::UDPclient | ModeTab::UDPserver => views::udp_client::view(self),
            _ => views::settings::view(self),
        };
        let content = column![tabs, col].height(Fill).spacing(20).max_width(800);
        // Create a container with a background image let background_image = Handle::from_bytes(APP_BACKGROUND); // Assuming APP_BACKGROUND is defined
        // let bg = hero_image();
        // let background = Container::new(content)
        //     .style(|v| iced::widget::container::Style::default().background(bg))
        //     .padding(40);
        center(content).padding(40).into()
    }

    fn render_tabs(&self) -> Row<'_, Msg> {
        let buttons = TABS.iter().map(|tab| {
            let active = &self.tab == tab;
            let button_style = move |theme: &Theme, status: Status| match active {
                true => button::primary(theme, Status::Hovered),
                false => button::primary(theme, status),
            };
            let label = text(String::from(tab)).width(Fill).center();
            button(label)
                .style(button_style)
                .on_press(Msg::TabChanged(tab.clone()))
                .width(Fill)
                .padding(8)
                .into()
        });
        Row::with_children(buttons).align_y(Center).spacing(10)
    }

    fn subscription(&self) -> Subscription<Msg> {
        let scan_sub = match self.scan_progress {
            255 => Subscription::none(),
            _ => views::ip_scan::subscription(),
        };
        let kb_sub = keyboard::on_key_press(Msg::key_press);
        Subscription::batch([scan_sub, kb_sub])
    }
}
