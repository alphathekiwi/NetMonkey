use std::net::{IpAddr, Ipv4Addr};

use iced::keyboard;
use iced::widget::{Row, button, center, checkbox, column, row, text, text_input};
use iced::window;
use iced::{Center, Element, Fill, Subscription, Task as Command};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
mod adaptor;
// use iced::widget;
// use iced::widget::{Text, keyed_column, scrollable};

pub fn main() -> iced::Result {
    #[cfg(not(target_arch = "wasm32"))]
    tracing_subscriber::fmt::init();

    iced::application("Todos - Iced", IpScannerApp::update, IpScannerApp::view)
        .subscription(IpScannerApp::subscription)
        .font(IpScannerApp::ICON_FONT)
        .window_size((500.0, 800.0))
        .run_with(IpScannerApp::new)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModeTab {
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
#[allow(unused)]
#[derive(Debug)]
struct ScannedIp {
    id: IpAddr,
    alive: bool,
    ports: Vec<u16>,
}
#[allow(unused)]
#[derive(Debug)]
struct IpScannerApp {
    tab: ModeTab,
    starting_ip: IpAddr,
    subnet_mask: u8,
    ports: Vec<u16>,
    ips: Vec<ScannedIp>,
}
impl Default for IpScannerApp {
    fn default() -> Self {
        Self {
            tab: ModeTab::IpScan,
            starting_ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
            subnet_mask: 24,
            ports: vec![80, 443],
            ips: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    TabChanged(ModeTab),
    TabPressed { shift: bool },
    ToggleFullscreen(window::Mode),
}

impl IpScannerApp {
    const ICON_FONT: &'static [u8] = include_bytes!("../fonts/icons.ttf");

    fn new() -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TabChanged(tab) => {
                self.tab = tab;
                Command::none()
            }
            Message::TabPressed { shift } => {
                if shift {
                    iced::widget::focus_previous()
                } else {
                    iced::widget::focus_next()
                }
            }
            Message::ToggleFullscreen(mode) => {
                window::get_latest().and_then(move |window| window::change_mode(window, mode))
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let tab_names = vec![
            ModeTab::IpScan,
            ModeTab::TCPclient,
            ModeTab::UDPclient,
            ModeTab::Settings,
        ];
        let buttons = tab_names.into_iter().map(|tab| {
            let label = text(String::from(&tab));
            let style = match self.tab == tab {
                true => button::primary,
                false => button::text,
            };
            let button = button(label).style(style);
            button
                .on_press(Message::TabChanged(tab.clone()))
                .padding(8)
                .into()
        });
        let tabs = Row::with_children(buttons).spacing(10);

        // let title = text("todos")
        //     .width(Fill)
        //     .size(100)
        //     .color([0.5, 0.5, 0.5])
        //     .align_x(Center);

        let content = column![tabs].spacing(20).max_width(800);
        center(content).padding(40).into()
    }

    fn subscription(&self) -> Subscription<Message> {
        use keyboard::key;

        keyboard::on_key_press(|key, modifiers| {
            let keyboard::Key::Named(key) = key else {
                return None;
            };

            match (key, modifiers) {
                (key::Named::Tab, _) => Some(Message::TabPressed {
                    shift: modifiers.shift(),
                }),
                (key::Named::ArrowUp, keyboard::Modifiers::SHIFT) => {
                    Some(Message::ToggleFullscreen(window::Mode::Fullscreen))
                }
                (key::Named::ArrowDown, keyboard::Modifiers::SHIFT) => {
                    Some(Message::ToggleFullscreen(window::Mode::Windowed))
                }
                _ => None,
            }
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    #[serde(default = "Uuid::new_v4")]
    id: Uuid,
    description: String,
    completed: bool,

    #[serde(skip)]
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

impl Task {
    fn new(description: String) -> Self {
        Task {
            id: Uuid::new_v4(),
            description,
            completed: false,
            state: TaskState::Idle,
        }
    }

    fn update(&mut self, message: TaskMessage) {
        match message {
            TaskMessage::Completed(completed) => {
                self.completed = completed;
            }
            TaskMessage::Edit => {
                self.state = TaskState::Editing;
            }
            TaskMessage::DescriptionEdited(new_description) => {
                self.description = new_description;
            }
            TaskMessage::FinishEdition => {
                if !self.description.is_empty() {
                    self.state = TaskState::Idle;
                }
            }
            TaskMessage::Delete => {}
        }
    }

    fn view(&self, i: usize) -> Element<'_, TaskMessage> {
        match &self.state {
            TaskState::Idle => {
                let checkbox = checkbox(&self.description, self.completed)
                    .on_toggle(TaskMessage::Completed)
                    .width(Fill)
                    .size(17)
                    .text_shaping(text::Shaping::Advanced);

                row![checkbox].spacing(20).align_y(Center).into()
            }
            TaskState::Editing => {
                let text_input = text_input("Describe your task...", &self.description)
                    .id("test")
                    .on_input(TaskMessage::DescriptionEdited)
                    .on_submit(TaskMessage::FinishEdition)
                    .padding(10);

                row![text_input].spacing(20).align_y(Center).into()
            }
        }
    }
}
