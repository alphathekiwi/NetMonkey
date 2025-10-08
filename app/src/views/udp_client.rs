use iced::Alignment::Center;
use iced::Length::{Fill, FillPortion};
use iced::widget::{Column, button, column, row, text, text_input};
use net_monkey_components::TextInputDropdown;

use crate::Msg;
use crate::views::settings::IpScannerApp;
use net_monkey_theme::helpers;

pub fn view<'a>(app: &'a IpScannerApp) -> Column<'a, Msg> {
    let theme_colors = app.config.theme.colors();

    let (connected_text, connected_color) = match app.udp_connection {
        None => ("Connect".to_string(), theme_colors.primary),
        Some(connection) => (format!("Disconnect from {connection}"), theme_colors.danger),
    };

    let connected = text(connected_text).color(connected_color);
    let history = app.udp_history.join("\n");

    // Determine history text color based on connection status
    let history_color = match app.udp_connection {
        None => theme_colors.text_secondary,
        Some(_) => theme_colors.success,
    };

    let items = app
        .ips
        .iter()
        .map(|scanned| scanned.ip.to_string())
        .collect::<Vec<String>>();
    let ip_sel: TextInputDropdown<_, _, Msg, iced::Theme> = TextInputDropdown::new(
        items,
        app.udp_connection.map_or_default(|c| c.to_string()),
        Msg::UdpIpAddress,
        Msg::UdpIpAddress,
    );
    // Create themed connection controls container
    let connection_controls = helpers::themed_container(
        row![
            ip_sel.text_size(24),
            row![
                text_input("Port", &app.udp_ip_port)
                    .on_input(Msg::UdpIpPort)
                    .size(24)
                    .width(FillPortion(1))
                    .padding(8),
                button(connected)
                    .on_press(Msg::UdpConnectionToggle)
                    .width(FillPortion(1))
                    .padding(8),
            ]
            .spacing(10),
        ]
        .spacing(10),
        app.config.theme.clone(),
        app.udp_connection.is_some(),
        false,
    );

    // Create themed history container
    let history_container = helpers::sub_menu_container(
        column![text(history).color(history_color).height(Fill),],
        app.config.theme.clone(),
    );

    // Create themed info panel
    let info_panel = helpers::menu_container(
        column![
            text("UDP Client/Server").size(24).color(theme_colors.text),
            text_input("Ip Address", "").size(24),
        ],
        app.config.theme.clone(),
    );

    let items = vec![
        connection_controls.into(),
        row![history_container, info_panel].spacing(10).into(),
    ];

    Column::with_children(items).align_x(Center).spacing(10)
}
