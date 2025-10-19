use iced::Alignment::Center;
use iced::Length::{Fill, FillPortion};
use iced::widget::{Column, button, column, row, text, text_input};
use net_monkey_components::TextInputDropdown;

use crate::Msg;
use crate::views::settings::IpScannerApp;
use net_monkey_theme::helpers;

pub fn view<'a>(app: &'a IpScannerApp) -> Column<'a, Msg> {
    let theme_colors = app.config.theme_provider().colors();

    let (connected_text, connected_color) = match app.tcp_connection {
        None => ("Connect", theme_colors.primary_color()),
        Some(_) => ("Disconnect", theme_colors.danger_color()),
    };

    let connected = text(connected_text).size(24).color(connected_color);

    let history = app.tcp_history.join("\n");

    // Determine history text color based on connection status
    let history_color = match app.tcp_connection {
        None => theme_colors.text_color(),
        Some(_) => theme_colors.success_color(),
    };

    let items = app
        .ips
        .iter()
        .map(|scanned| scanned.ip.to_string())
        .collect::<Vec<String>>();
    let ip_sel: TextInputDropdown<_, _, Msg, iced::Theme> = TextInputDropdown::new(
        items,
        app.tcp_connection.map_or_default(|c| c.to_string()),
        Msg::TcpIpAddress,
        Msg::TcpIpAddress,
    );
    // Create themed connection controls container
    let connection_controls = helpers::themed_container(
        row![
            ip_sel.text_size(24),
            row![
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
            .width(Fill),
        ]
        .align_y(Center)
        .spacing(15)
        .width(Fill),
        &app.config.theme_provider(),
    );

    // Create themed history container
    let history_container = helpers::sub_menu_container(
        column![text(history).color(history_color).height(Fill),],
        &app.config.theme_provider(),
    );

    // Create themed info panel
    let info_panel = helpers::menu_container(
        column![
            text("TCP Client/Server")
                .size(24)
                .color(theme_colors.text_color()),
            text_input("Ip Address", "").size(24),
        ],
        &app.config.theme_provider(),
    );

    let items = vec![
        connection_controls.into(),
        row![history_container, info_panel].spacing(10).into(),
    ];

    Column::with_children(items).align_x(Center).spacing(10)
}
