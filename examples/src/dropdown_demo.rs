use iced::widget::{column, container, text};
use iced::{Element, Length, Task};

use net_monkey_components::TextInputDropdown;

#[derive(Debug, Clone)]
pub enum Message {
    DropdownChanged(String),
    DropdownSelected(String),
}

pub struct DropdownDemo {
    selected_value: String,
    dropdown_items: Vec<String>,
}

impl Default for DropdownDemo {
    fn default() -> Self {
        Self {
            selected_value: String::from("192.168.1.1"),
            dropdown_items: vec![
                "192.168.1.1".to_string(),
                "192.168.1.100".to_string(),
                "192.168.1.254".to_string(),
                "10.0.0.1".to_string(),
                "10.0.0.254".to_string(),
                "172.16.0.1".to_string(),
                "172.16.0.254".to_string(),
                "127.0.0.1".to_string(),
            ],
        }
    }
}

impl DropdownDemo {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::DropdownChanged(value) => {
                self.selected_value = value;
            }
            Message::DropdownSelected(value) => {
                self.selected_value = value;
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let dropdown = TextInputDropdown::new(
            self.dropdown_items.clone(),
            self.selected_value.clone(),
            Message::DropdownChanged,
            Message::DropdownSelected,
        )
        .text_size(20);

        let content = column![
            text("Text Input Dropdown Demo").size(24),
            text("Type to filter or select from the dropdown:").size(16),
            text("Available IP Addresses:").size(14),
            dropdown,
            text("").size(10), // Spacer
            text("Current Selection:").size(18),
            container(text(&self.selected_value).size(16))
                .padding(10)
                .style(|theme: &iced::Theme| container::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgb(
                        0.1, 0.1, 0.15,
                    ))),
                    border: iced::Border {
                        color: iced::Color::from_rgb(0.3, 0.3, 0.4),
                        width: 1.0,
                        radius: 8.0.into(),
                    },
                    ..Default::default()
                }),
            text("").size(10), // Spacer
            text("Instructions:").size(16),
            text("• Type in the field to enter a custom IP").size(12),
            text("• Click the dropdown arrow to see all options").size(12),
            text("• Click on any dropdown item to select it").size(12),
            text("• The field supports both typing and selection").size(12),
        ]
        .spacing(10)
        .padding(20)
        .max_width(400);

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
        "Text Input Dropdown Demo",
        DropdownDemo::update,
        DropdownDemo::view,
    )
    .window_size((500.0, 600.0))
    .run_with(|| (DropdownDemo::default(), Task::none()))
}
