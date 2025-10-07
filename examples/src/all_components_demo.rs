use iced::widget::{column, container, row, text};
use iced::{Element, Length, Task, Theme};

use net_monkey_components::{
    LabelWithHint, SubnetSlider, TextInputDropdown, TextInputWithHint, text_input_with_hint,
};
use net_monkey_theme::NetMonkeyTheme;

#[derive(Debug, Clone)]
pub enum Message {
    IpAddressChanged(String),
    PortChanged(String),
    SubnetChanged(u8),
    DropdownChanged(String),
    DropdownSelected(String),
    ThemeChanged(NetMonkeyTheme),
}

pub struct AllComponentsDemo {
    ip_address: String,
    port: String,
    subnet_mask: u8,
    dropdown_value: String,
    current_theme: NetMonkeyTheme,
    dropdown_items: Vec<String>,
}

impl Default for AllComponentsDemo {
    fn default() -> Self {
        Self {
            ip_address: String::from("192.168.1.1"),
            port: String::from("80"),
            subnet_mask: 24,
            dropdown_value: String::from("192.168.1.1"),
            current_theme: NetMonkeyTheme::Dark,
            dropdown_items: vec![
                "192.168.1.1".to_string(),
                "192.168.1.100".to_string(),
                "10.0.0.1".to_string(),
                "172.16.0.1".to_string(),
            ],
        }
    }
}

impl AllComponentsDemo {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::IpAddressChanged(value) => {
                self.ip_address = value;
            }
            Message::PortChanged(value) => {
                self.port = value;
            }
            Message::SubnetChanged(value) => {
                self.subnet_mask = value;
            }
            Message::DropdownChanged(value) => {
                self.dropdown_value = value;
            }
            Message::DropdownSelected(value) => {
                self.dropdown_value = value;
            }
            Message::ThemeChanged(theme) => {
                self.current_theme = theme;
            }
        }
        Task::none()
    }

    fn theme(&self) -> Theme {
        self.current_theme.into()
    }

    fn view(&self) -> Element<'_, Message> {
        // Theme selector
        let theme_selector = row![
            text("Theme:").size(16),
            container(text(format!("{}", self.current_theme)).size(14))
                .padding(8)
                .style(|theme: &Theme| {
                    container::Style {
                        background: Some(iced::Background::Color(iced::Color::from_rgb(
                            0.2, 0.2, 0.3,
                        ))),
                        border: iced::Border {
                            color: iced::Color::from_rgb(0.4, 0.4, 0.5),
                            width: 1.0,
                            radius: 4.0.into(),
                        },
                        ..Default::default()
                    }
                })
        ]
        .spacing(10)
        .align_y(iced::Alignment::Center);

        // Text Input with Hint
        let ip_input = text_input_with_hint(
            self.ip_address.clone(),
            "Enter IP Address",
            "Example: 192.168.1.1 - Enter a valid IPv4 address",
            Message::IpAddressChanged,
        )
        .text_size(18.0)
        .theme(self.current_theme)
        .into_element();

        let port_input = TextInputWithHint::new(
            self.port.clone(),
            "Enter Port",
            "Port range: 1-65535 - Common ports: 80 (HTTP), 443 (HTTPS), 22 (SSH)",
            Message::PortChanged,
        )
        .text_size(18.0)
        .theme(self.current_theme)
        .into_element();

        // Label with Hint
        let ip_label = LabelWithHint::new(
            "IP Address:",
            "The target IP address for network operations",
        )
        .text_size(16.0)
        .theme(self.current_theme)
        .into_element();

        let port_label = LabelWithHint::new("Port:", "The target port number (1-65535)")
            .text_size(16.0)
            .theme(self.current_theme)
            .into_element();

        // Subnet Slider
        let subnet_label = LabelWithHint::new(
            "Subnet Mask:",
            "CIDR notation - determines network size. /24 = 254 hosts, /16 = 65534 hosts",
        )
        .text_size(16.0)
        .theme(self.current_theme)
        .into_element();

        let subnet_slider = SubnetSlider::new(self.subnet_mask, Message::SubnetChanged)
            .text_size(18.0)
            .height(50.0)
            .into_element();

        // Dropdown
        let dropdown_label = LabelWithHint::new(
            "Network Adapter:",
            "Select from available network adapters or enter custom IP",
        )
        .text_size(16.0)
        .theme(self.current_theme)
        .into_element();

        let dropdown = TextInputDropdown::new(
            self.dropdown_items.clone(),
            self.dropdown_value.clone(),
            Message::DropdownChanged,
            Message::DropdownSelected,
        )
        .text_size(18);

        // Network info display
        let host_count = 2_u32.pow(32 - self.subnet_mask as u32) - 2;
        let network_info = container(
            column![
                text("Current Configuration:").size(18),
                text(format!("IP Address: {}", self.ip_address)).size(14),
                text(format!("Port: {}", self.port)).size(14),
                text(format!(
                    "Subnet: /{} ({} hosts)",
                    self.subnet_mask, host_count
                ))
                .size(14),
                text(format!("Selected Adapter: {}", self.dropdown_value)).size(14),
            ]
            .spacing(5),
        )
        .padding(15)
        .style(|theme: &Theme| container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgb(
                0.1, 0.1, 0.15,
            ))),
            border: iced::Border {
                color: iced::Color::from_rgb(0.3, 0.3, 0.4),
                width: 1.0,
                radius: 8.0.into(),
            },
            ..Default::default()
        });

        let content = column![
            text("Net Monkey Components Demo").size(28),
            text("Showcasing all custom UI components with theming").size(16),
            theme_selector,
            text("").size(10), // Spacer
            ip_label,
            ip_input,
            text("").size(10), // Spacer
            port_label,
            port_input,
            text("").size(10), // Spacer
            subnet_label,
            subnet_slider,
            text("").size(10), // Spacer
            dropdown_label,
            dropdown,
            text("").size(15), // Spacer
            network_info,
        ]
        .spacing(8)
        .padding(30)
        .max_width(600);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}

pub fn main() -> iced::Result {
    iced::application(
        "Net Monkey Components Demo",
        AllComponentsDemo::update,
        AllComponentsDemo::view,
    )
    .theme(|demo: &AllComponentsDemo| demo.current_theme.into())
    .window_size((800.0, 900.0))
    .run_with(|| (AllComponentsDemo::default(), Task::none()))
}
