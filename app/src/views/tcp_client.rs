use iced::Alignment::Center;
use iced::Length::{Fill, FillPortion};
use iced::widget::{Column, button, column, row, scrollable, text, text_input};
use net_monkey_components::TextInputDropdown;

use crate::Msg;
use crate::views::settings::IpScannerApp;
use net_monkey_theme::helpers;

pub fn view<'a>(app: &'a IpScannerApp) -> Column<'a, Msg> {
    let theme_colors = app.config.theme_provider().colors();

    let (connected_text, connected_color) = match app.tcp_client.connections.is_empty() {
        true => ("Connect", theme_colors.primary_color()),
        false => ("Disconnect", theme_colors.danger_color()),
    };

    let connected = text(connected_text).size(24).color(connected_color);

    let history = app.tcp_client.history.join("\n");

    let items = app
        .ips
        .iter()
        .map(|scanned| scanned.ip.to_string())
        .collect::<Vec<String>>();
    let ip_sel: TextInputDropdown<_, _, Msg, iced::Theme> = TextInputDropdown::new(
        items,
        app.tcp_client
            .connections
            .first()
            .map_or_default(|c| c.to_string()),
        Msg::ChangeIpAddress,
        Msg::ChangeIpAddress,
    );
    // Create themed connection controls container
    let connection_controls = helpers::themed_container(
        row![
            ip_sel.text_size(24),
            row![
                text_input("Port", &app.tcp_client.ip_port)
                    .on_input(Msg::ChangeIpPort)
                    .size(24)
                    .width(Fill)
                    .padding(8),
                button(connected)
                    .on_press(Msg::ConnectionToggle)
                    .width(Fill)
                    .height(Fill)
                    .padding(8),
            ]
            .spacing(15)
            .width(Fill),
        ]
        .align_y(Center)
        .spacing(15)
        .width(Fill),
        &app.config.theme_provider(),
    );

    // Create themed history container with scrollable content
    let history_container = helpers::sub_menu_container(
        scrollable(text(history).color(theme_colors.text_color()).width(Fill))
            .height(Fill)
            .width(Fill),
        &app.config.theme_provider(),
    )
    .height(Fill);

    let packet_sending = helpers::themed_container(
        row![
            text_input("Message to socket", &app.tcp_client.current_packet)
                .on_input(Msg::ChangePacket)
                .size(24)
                .width(FillPortion(3))
                .padding(8),
            button(text("Send Packet").size(24))
                .on_press(Msg::SendPacket)
                .width(FillPortion(1))
                .height(Fill)
                .padding(8),
        ]
        .align_y(Center)
        .spacing(15)
        .width(Fill),
        &app.config.theme_provider(),
    );

    column![connection_controls, history_container, packet_sending]
        .align_x(Center)
        .spacing(10)
        .height(Fill)
}
