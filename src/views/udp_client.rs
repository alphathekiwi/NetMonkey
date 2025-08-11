use iced::Alignment::Center;
use iced::Color;
use iced::Length::Fill;
use iced::widget::{Column, button, column, row, text, text_input};

use crate::Msg;
use crate::views::settings::IpScannerApp;

pub fn view<'a>(app: &'a IpScannerApp) -> Column<'a, Msg> {
    let connected = match app.udp_connection {
        None => text("Connect"),
        Some(connection) => text(format!("Disconnect from {connection}")),
    };
    let history = app.udp_history.join("\n");
    let items = vec![
        row![
            text_input("Ip Address", &app.udp_ip_address)
                .on_input(Msg::UdpIpAddress)
                .size(24),
            text_input("Port", &app.udp_ip_port)
                .on_input(Msg::UdpIpPort)
                .size(24),
            button(connected).on_press(Msg::UdpConnectionToggle),
        ]
        .into(),
        row![
            column![
                text(history)
                    .color(Color::from_rgb(1.0, 0.5, 0.5))
                    .height(Fill),
            ],
            column![
                text("TCP Client/Server").size(24),
                text_input("Ip Address", "").size(24),
            ]
        ]
        .into(),
    ];

    Column::with_children(items).align_x(Center).spacing(10)
}
