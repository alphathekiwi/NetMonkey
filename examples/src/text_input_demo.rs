use iced::widget::{column, container, text};
use iced::{Element, Length, Task};

use net_monkey_components::text_input_with_hint;

#[derive(Debug, Clone)]
pub enum Message {
    IpAddressChanged(String),
    PortChanged(String),
    SubnetChanged(String),
}

pub struct TextInputDemo {
    ip_address: String,
    port: String,
    subnet: String,
}

impl Default for TextInputDemo {
    fn default() -> Self {
        Self {
            ip_address: String::new(),
            port: String::new(),
            subnet: String::new(),
        }
    }
}

impl TextInputDemo {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::IpAddressChanged(value) => {
                self.ip_address = value;
            }
            Message::PortChanged(value) => {
                self.port = value;
            }
            Message::SubnetChanged(value) => {
                self.subnet = value;
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let ip_input = text_input_with_hint(
            self.ip_address.clone(),
            "Enter IP Address",
            "Example: 192.168.1.1",
            Message::IpAddressChanged,
        )
        .into_element();

        let port_input = text_input_with_hint(
            self.port.clone(),
            "Enter Port",
            "Port range: 1-65535",
            Message::PortChanged,
        )
        .into_element();

        let subnet_input = text_input_with_hint(
            self.subnet.clone(),
            "Enter Subnet",
            "CIDR notation: /24",
            Message::SubnetChanged,
        )
        .into_element();

        let content = column![
            text("Text Input with Hint Demo").size(24),
            text("IP Address:").size(16),
            ip_input,
            text("Port:").size(16),
            port_input,
            text("Subnet:").size(16),
            subnet_input,
            text("Current Values:").size(18),
            text(format!("IP: {}", self.ip_address)).size(14),
            text(format!("Port: {}", self.port)).size(14),
            text(format!("Subnet: {}", self.subnet)).size(14),
        ]
        .spacing(10)
        .padding(20);

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
        "Text Input with Hint Demo",
        TextInputDemo::update,
        TextInputDemo::view,
    )
    .run_with(|| (TextInputDemo::default(), Task::none()))
}
