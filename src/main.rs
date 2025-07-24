// #![allow(unused_imports, unused)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::keyboard::key::Named;
use iced::keyboard::{Key, Modifiers};
use iced::widget::button::Status;
use iced::widget::image as iced_image;
use iced::widget::image::Handle;
use iced::widget::{Image, Row, button, center, column, text};
use iced::window::icon::from_file_data;
use iced::window::{Mode, Settings};
use iced::{Center, Element, Fill, Subscription, Task as Command, Theme, keyboard, window};
use image::ImageFormat;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::views::ip_scan::{self, ScannedIp};
use crate::views::settings::{IpScannerApp, ModeTab};

mod adaptor;
mod views;

pub fn main() -> iced::Result {
    #[cfg(not(target_arch = "wasm32"))]
    tracing_subscriber::fmt::init();

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
    TabChanged(ModeTab),
    TabPressed { shift: bool },
    ToggleFullscreen(Mode),
    BeginScan,
    ScanComplete,
    PingResult(ScannedIp),
}
impl Msg {
    fn key_press(any_key: Key, mods: Modifiers) -> Option<Msg> {
        let Key::Named(key) = any_key else {
            return None;
        };
        match (key, mods) {
            (Named::ArrowUp, Modifiers::SHIFT) => Some(Msg::ToggleFullscreen(Mode::Fullscreen)),
            (Named::ArrowDown, Modifiers::SHIFT) => Some(Msg::ToggleFullscreen(Mode::Windowed)),
            (Named::Tab, _) => Some(Msg::tab(mods.shift())),
            _ => None,
        }
    }
    fn tab(shift: bool) -> Self {
        Self::TabPressed { shift }
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
        (Self::default(), Command::none())
    }

    fn update(&mut self, message: Msg) -> Command<Msg> {
        match message {
            Msg::TabChanged(tab) => {
                self.tab = tab;
                Command::none()
            }
            Msg::TabPressed { shift } => {
                if shift {
                    iced::widget::focus_previous()
                } else {
                    iced::widget::focus_next()
                }
            }
            Msg::ToggleFullscreen(mode) => {
                window::get_latest().and_then(move |window| window::change_mode(window, mode))
            }
            Msg::PingResult(res) => {
                // Add IP to your UI list immediately
                self.ips.push(res);
                Command::none()
            }
            Msg::BeginScan => {
                println!("Starting scan...");
                self.scan_progress = 1;
                Command::none()
            }
            Msg::ScanComplete => {
                println!("Scan completed!");
                self.scan_progress = 0;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Msg> {
        let tabs = self.render_tabs();

        let col = match self.tab {
            ModeTab::IpScan => ip_scan::view(&self.ips),
            ModeTab::TCPclient | ModeTab::TCPserver => views::tcp_client::view(),
            ModeTab::UDPclient | ModeTab::UDPserver => views::udp_client::view(),
            _ => views::settings::view(),
        };
        let content = column![tabs, col].height(Fill).spacing(20).max_width(800);

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
            0 => Subscription::none(),
            _ => ip_scan::subscription(),
        };
        let kb_sub = keyboard::on_key_press(Msg::key_press);
        Subscription::batch([scan_sub, kb_sub])
    }
}
// The subscription function

// async fn ping_ip(ip: IpAddr) -> anyhow::Result<()> {
//     let timeout = Some(Duration::from_millis(1000));
//     ping::ping(ip, timeout, None, None, None, None)?;
//     Ok(())
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    #[serde(default = "Uuid::new_v4")]
    id: Uuid,
    description: String,
    completed: bool,

    #[serde(skip)]
    #[allow(unused)]
    state: TaskState,
}

#[derive(Debug, Clone)]
pub enum TaskState {
    Idle,
    Editing,
}

impl Default for TaskState {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Debug, Clone)]
pub enum TaskMessage {
    Completed(bool),
    Edit,
    DescriptionEdited(String),
    FinishEdition,
    Delete,
}
