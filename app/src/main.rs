// #![allow(unused_imports, unused)]
#![feature(addr_parse_ascii, result_option_map_or_default)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::net::IpAddr;

use iced::keyboard::{Key, Modifiers, key::Named};
use iced::widget::image::Handle;
use iced::widget::{Image, Row, button, center, column, container, text};
use iced::widget::{button::Status, image as iced_image};
use iced::window::{Mode, Settings, icon::from_file_data};
use iced::{Center, Color, Element, Fill, Subscription, Task as Command, Theme, keyboard, window};
use image::ImageFormat;

use crate::views::settings::{AppConfig, ChangeConfig, IpScannerApp, ModeTab};
use net_monkey_core::{NetworkAdapter, ScannedIp, get_network_adapters};
use net_monkey_theme::helpers;

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
    FocusMove {
        shift: bool,
    },
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
    ColorEdit {
        color_type: crate::views::settings::ColorType,
        hex_value: String,
    },
    SaveTheme,
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
            .theme(Self::theme)
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
            Msg::ColorEdit {
                color_type,
                hex_value,
            } => self
                .config
                .update(ChangeConfig::ColorChange(color_type, hex_value)),
            Msg::SaveTheme => {
                // Save current theme as a new permanent theme
                use net_monkey_theme::{NetMonkeyTheme, ThemeDefinition, ThemeManager};
                use std::time::{SystemTime, UNIX_EPOCH};

                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();

                let current_colors = self.config.theme.clone().colors();
                let theme_def = ThemeDefinition {
                    name: format!("Custom Theme {timestamp}"),
                    description: "User created custom theme".to_string(),
                    colors: current_colors,
                    is_dark: current_colors.background.r < 0.5,
                };
                if let Err(e) = ThemeManager::save_theme(&theme_def) {
                    eprintln!("Failed to save theme: {e}");
                } else {
                    println!("Theme saved successfully: {}", theme_def.name);
                    // Switch to the newly saved theme
                    self.config.theme = NetMonkeyTheme::Loaded(theme_def.name.clone());
                }
            }
            _ => {}
        }
        cmd
    }

    fn theme(&self) -> Theme {
        self.config.theme.clone().to_extended_iced_theme()
    }

    fn view(&self) -> Element<'_, Msg> {
        let theme_colors = self.config.theme.clone().colors();
        let tabs = self.render_tabs();
        let col = match self.tab {
            ModeTab::IpScan => views::ip_scan::view(self).into(),
            ModeTab::TCPclient | ModeTab::TCPserver => views::tcp_client::view(self).into(),
            ModeTab::UDPclient | ModeTab::UDPserver => views::udp_client::view(self).into(),
            ModeTab::ThemeEdit => views::theme_edit::view(self),
            _ => views::settings::view(self),
        };

        // Create themed content container
        let content = helpers::menu_container(
            column![tabs, col].height(Fill).spacing(20),
            self.config.theme.clone(),
        );

        // Main background container with theme colors
        let background = container(center(content).padding(40))
            .style(move |_| container::Style {
                background: Some(iced::Background::Color(theme_colors.background.into())),
                ..Default::default()
            })
            .height(Fill)
            .width(Fill);

        background.into()
    }

    fn render_tabs(&self) -> Row<'_, Msg> {
        let theme_colors = self.config.theme.clone().colors();
        let buttons = TABS.iter().map(|tab| {
            let active = &self.tab == tab;
            let button_style = move |theme: &Theme, status: Status| {
                let base_style = button::primary(theme, status);
                match (active, status) {
                    (true, _) => button::Style {
                        background: Some(iced::Background::Color(theme_colors.active.into())),
                        text_color: theme_colors.text.into(),
                        border: iced::Border {
                            color: theme_colors.border_focused.into(),
                            width: 2.0,
                            radius: 4.0.into(),
                        },
                        ..base_style
                    },
                    (false, Status::Hovered) => button::Style {
                        background: Some(iced::Background::Color(theme_colors.hover.into())),
                        text_color: theme_colors.text.into(),
                        border: iced::Border {
                            color: theme_colors.border_hover.into(),
                            width: 1.0,
                            radius: 4.0.into(),
                        },
                        ..base_style
                    },
                    _ => button::Style {
                        background: Some(iced::Background::Color(theme_colors.sub_menu.into())),
                        text_color: theme_colors.text.into(),
                        border: iced::Border {
                            color: theme_colors.border.into(),
                            width: 1.0,
                            radius: 4.0.into(),
                        },
                        ..base_style
                    },
                }
            };
            let label = text(String::from(tab))
                .width(Fill)
                .center()
                .color(Color::from(theme_colors.text));
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
