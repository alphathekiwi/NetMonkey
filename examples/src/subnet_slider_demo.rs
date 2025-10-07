use iced::widget::{column, container, text};
use iced::{Element, Length, Task};

use net_monkey_components::SubnetSlider;

#[derive(Debug, Clone)]
pub enum Message {
    SubnetChanged(u8),
}

pub struct SubnetSliderDemo {
    subnet_mask: u8,
}

impl Default for SubnetSliderDemo {
    fn default() -> Self {
        Self { subnet_mask: 24 }
    }
}

impl SubnetSliderDemo {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SubnetChanged(value) => {
                self.subnet_mask = value;
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let slider = SubnetSlider::new(self.subnet_mask, Message::SubnetChanged)
            .text_size(20.0)
            .height(50.0)
            .into_element();

        // Calculate network info based on subnet mask
        let host_count = 2_u32.pow(32 - self.subnet_mask as u32) - 2;
        let network_count = 2_u32.pow(self.subnet_mask as u32 - 8); // Assuming Class C

        let content = column![
            text("Subnet Slider Demo").size(24),
            text("Adjust the subnet mask using the slider below:").size(16),
            slider,
            text(format!("Current Subnet Mask: /{}", self.subnet_mask)).size(18),
            text(format!("Hosts per network: {}", host_count)).size(14),
            text(format!("Number of subnets: {}", network_count)).size(14),
            text("Subnet mask in dotted decimal:").size(14),
            text(format!(
                "{}",
                subnet_mask_to_dotted_decimal(self.subnet_mask)
            ))
            .size(14),
        ]
        .spacing(15)
        .padding(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}

fn subnet_mask_to_dotted_decimal(cidr: u8) -> String {
    let mask = 0xFFFFFFFFu32 << (32 - cidr);
    format!(
        "{}.{}.{}.{}",
        (mask >> 24) & 0xFF,
        (mask >> 16) & 0xFF,
        (mask >> 8) & 0xFF,
        mask & 0xFF
    )
}

pub fn main() -> iced::Result {
    iced::application(
        "Subnet Slider Demo",
        SubnetSliderDemo::update,
        SubnetSliderDemo::view,
    )
    .run_with(|| (SubnetSliderDemo::default(), Task::none()))
}
