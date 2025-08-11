use iced::Alignment::Center;
use iced::Color;
use iced::Length::{Fill, FillPortion};
use iced::widget::{Column, button, column, row, text, text_input};

use crate::Msg;
use crate::views::settings::IpScannerApp;

pub fn view<'a>(app: &'a IpScannerApp) -> Column<'a, Msg> {
    let connected = match app.tcp_connection {
        None => text("Connect").size(24),
        Some(_) => text("Disconnect").size(24),
    };
    let history = app.tcp_history.join("\n");
    let items = vec![
        row![
            text_input("Ip Address", &app.tcp_ip_address)
                .on_input(Msg::TcpIpAddress)
                .size(24)
                .width(FillPortion(2))
                .padding(8),
            text_input("Port", &app.tcp_ip_port)
                .on_input(Msg::TcpIpPort)
                .size(24)
                .width(FillPortion(1))
                .padding(8),
            button(connected)
                .on_press(Msg::TcpConnectionToggle)
                .width(FillPortion(1))
                .padding(8),
        ]
        .spacing(10)
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
