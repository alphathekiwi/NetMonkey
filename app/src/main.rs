// #![allow(unused_imports, unused)]
#![feature(addr_parse_ascii, result_option_map_or_default)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::net::IpAddr;

#[cfg(feature = "cosmic")]
use cosmic::app::{Core, Settings, Task};
#[cfg(feature = "cosmic")]
use cosmic::iced_core::Size;
#[cfg(feature = "cosmic")]
use cosmic::keyboard::{Key, Modifiers, key::Named};
#[cfg(feature = "cosmic")]
use cosmic::widget::image::Handle;
#[cfg(feature = "cosmic")]
use cosmic::widget::{Image, Row, button, center, column, container, text};
#[cfg(feature = "cosmic")]
use cosmic::widget::{button::Status, image as iced_image};
#[cfg(feature = "cosmic")]
use cosmic::window::{Mode, icon::from_file_data};
#[cfg(feature = "cosmic")]
use cosmic::{ApplicationExt, Center, Color, Element, Fill, Subscription, keyboard, window};
#[cfg(feature = "cosmic")]
use image::ImageFormat;

#[cfg(not(feature = "cosmic"))]
use iced::keyboard::{Key, Modifiers, key::Named};
#[cfg(not(feature = "cosmic"))]
use iced::widget::image::Handle;
#[cfg(not(feature = "cosmic"))]
use iced::widget::{Image, Row, button, center, column, container, text};
#[cfg(not(feature = "cosmic"))]
use iced::widget::{button::Status, image as iced_image};
#[cfg(not(feature = "cosmic"))]
use iced::window::{Mode, Settings, icon::from_file_data};
#[cfg(not(feature = "cosmic"))]
use iced::{Center, Color, Element, Fill, Subscription, Task, Theme, keyboard};
#[cfg(not(feature = "cosmic"))]
use image::ImageFormat;

use crate::views::settings::{AppConfig, ChangeConfig, IpScannerApp, ModeTab};
use net_monkey_core::{NetworkAdapter, ScannedIp, get_network_adapters};
use net_monkey_theme::helpers;

mod views;

#[cfg(feature = "cosmic")]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::default().size(Size::new(500.0, 800.0));

    let input = (
        AppConfig::load().unwrap_or_default(),
        get_network_adapters(),
    );

    cosmic::app::run::<IpScannerApp>(settings, input)?;
    Ok(())
}

#[cfg(not(feature = "cosmic"))]
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
    RefreshTheme,
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

#[cfg(feature = "cosmic")]
impl cosmic::Application for IpScannerApp {
    type Executor = cosmic::executor::Default;
    type Flags = (AppConfig, Vec<NetworkAdapter>);
    type Message = Msg;
    const APP_ID: &'static str = "com.system76.NetMonkey";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, (config, adapters): Self::Flags) -> (Self, Task<Self::Message>) {
        let mut app = Self {
            core,
            config,
            adapters,
            ..Default::default()
        };
        app.loaded(config.clone(), adapters.clone());
        (app, Task::none())
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        self.update_common(message)
    }

    fn view(&self) -> Element<Self::Message> {
        self.view_common()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        self.subscription_common()
    }
}

#[cfg(not(feature = "cosmic"))]
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

    #[cfg(not(feature = "cosmic"))]
    fn initialize() -> (Self, Task<Msg>) {
        (
            Self::default(),
            Task::perform(
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
}

impl IpScannerApp {
    fn update_common(&mut self, msg: Msg) -> Task<Msg> {
        #[cfg(feature = "cosmic")]
        use cosmic::widget::{focus_next, focus_previous};
        #[cfg(feature = "cosmic")]
        use cosmic::window::{change_mode, get_latest};
        #[cfg(not(feature = "cosmic"))]
        use iced::widget::{focus_next, focus_previous};
        #[cfg(not(feature = "cosmic"))]
        use iced::window::{change_mode, get_latest};

        // All Msgs that return a Task
        let cmd = match &msg {
            Msg::WinSize(mode) => {
                let mode = *mode; // Copy the mode value
                get_latest().and_then(move |id| change_mode(id, mode))
            }
            Msg::FocusMove { shift: true } => focus_previous(),
            Msg::FocusMove { shift: false } => focus_next(),
            _ => Task::none(),
        };

        self.update_state(msg);
        cmd
    }

    fn update_state(&mut self, msg: Msg) {
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
            Msg::RefreshTheme => {
                // Theme provider is created on-demand, no need to refresh
                println!("Theme refreshed");
            }
            _ => {}
        }
    }

    #[cfg(not(feature = "cosmic"))]
    fn update(&mut self, msg: Msg) -> Task<Msg> {
        self.update_common(msg)
    }

    #[cfg(not(feature = "cosmic"))]
    fn theme(&self) -> Theme {
        self.config.theme_provider().to_iced_theme()
    }

    #[cfg(not(feature = "cosmic"))]
    fn view(&self) -> Element<'_, Msg> {
        self.view_common()
    }

    fn subscription_common(&self) -> Subscription<Msg> {
        let scan_sub = match self.scan_progress {
            255 => Subscription::none(),
            _ => views::ip_scan::subscription(),
        };
        let kb_sub = keyboard::on_key_press(Msg::key_press);
        Subscription::batch([scan_sub, kb_sub])
    }

    #[cfg(not(feature = "cosmic"))]
    fn subscription(&self) -> Subscription<Msg> {
        self.subscription_common()
    }

    fn view_common(&self) -> Element<'_, Msg> {
        let colors = self.config.theme_provider().colors();
        let tabs = self.render_tabs();
        let col = match self.tab {
            ModeTab::IpScan => views::ip_scan::view(self).into(),
            ModeTab::TCPclient | ModeTab::TCPserver => views::tcp_client::view(self).into(),
            ModeTab::UDPclient | ModeTab::UDPserver => views::udp_client::view(self).into(),
            _ => views::settings::view(self),
        };

        // Create themed content container
        let content = helpers::themed_container(
            column![tabs, col].height(Fill).spacing(20),
            &self.config.theme_provider(),
        );

        // Main background container with theme colors
        let background = container(center(content).padding(40))
            .style(move |_| container::Style {
                background: Some(iced::Background::Color(colors.background_color())),
                ..Default::default()
            })
            .height(Fill)
            .width(Fill);

        background.into()
    }

    fn render_tabs(&self) -> Row<'_, Msg> {
        let colors = self.config.theme_provider().colors();
        let buttons = TABS.iter().map(|tab| {
            let active = &self.tab == tab;
            let primary_color = colors.primary_color();
            let background_color = colors.background_color();
            let text_color = colors.text_color();

            let button_style = move |_theme: &_, status: Status| match (active, status) {
                (true, _) => button::Style {
                    background: Some(iced::Background::Color(primary_color)),
                    text_color: background_color,
                    border: iced::Border {
                        color: primary_color,
                        width: 2.0,
                        radius: 4.0.into(),
                    },
                    ..Default::default()
                },
                (false, Status::Hovered) => button::Style {
                    background: Some(iced::Background::Color(Color::from_rgba(
                        primary_color.r,
                        primary_color.g,
                        primary_color.b,
                        0.2,
                    ))),
                    text_color,
                    border: iced::Border {
                        color: primary_color,
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    ..Default::default()
                },
                _ => button::Style {
                    background: Some(iced::Background::Color(Color::from_rgba(
                        text_color.r,
                        text_color.g,
                        text_color.b,
                        0.1,
                    ))),
                    text_color,
                    border: iced::Border {
                        color: Color::from_rgba(text_color.r, text_color.g, text_color.b, 0.3),
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    ..Default::default()
                },
            };
            let label = text(String::from(tab))
                .width(Fill)
                .center()
                .color(text_color);
            button(label)
                .style(button_style)
                .on_press(Msg::TabChanged(tab.clone()))
                .width(Fill)
                .padding(8)
                .into()
        });
        Row::with_children(buttons).align_y(Center).spacing(10)
    }
}
