use futures::StreamExt;
use iced::widget::Column;
use iced::widget::{button, column, progress_bar, row, stack, text};
use iced::{Element, Fill, Subscription};

use crate::views::settings::IpScannerApp;
use crate::{Msg, hero_image};
use net_monkey_core::{ScanMessage, ScannedIp, create_network_scanner};
use net_monkey_theme::helpers;

pub fn view(app: &IpScannerApp) -> Column<'_, Msg> {
    let theme_colors = app.config.theme_provider().colors();
    if app.ips.is_empty() {
        let mut scan_button = button(
            text("Scan Network")
                .width(Fill)
                .center()
                .size(20)
                .color(theme_colors.text_color()),
        )
        .style(button::primary)
        .width(Fill)
        .padding(12);

        if app.loaded {
            scan_button = scan_button.on_press(Msg::BeginScan);
        }

        let status_text = if app.loaded {
            text("If I were a grease monkey\nwhy would I need this net?")
                .width(Fill)
                .center()
                .size(30)
                .color(theme_colors.text_color())
        } else {
            text("Loading network configuration...")
                .width(Fill)
                .center()
                .size(24)
                .color(theme_colors.warning_color())
        };

        let welcome_container = helpers::menu_container(
            column![stack!(hero_image(), scan_button), status_text].spacing(20),
            &app.config.theme_provider(),
        );

        column![welcome_container]
    } else {
        let ping = app.ips.iter().map(|ip| ip.ping_elem(theme_colors));
        let ips = app.ips.iter().map(|ip| ip.ips_elem(theme_colors));
        let ports = app.ips.iter().map(|ip| ip.ports_elem(theme_colors));

        let progress_container = helpers::sub_menu_container(
            progress_bar(0.0..=255.0, app.scan_progress as f32),
            &app.config.theme_provider(),
        );

        let results_container = helpers::menu_container(
            row![
                helpers::sub_menu_container(
                    column![
                        text("Ping (ms)").size(16),
                        Column::with_children(ping).spacing(5)
                    ]
                    .spacing(10),
                    &app.config.theme_provider(),
                ),
                helpers::sub_menu_container(
                    column![
                        text("IP Address").size(16),
                        Column::with_children(ips).spacing(5)
                    ]
                    .spacing(10),
                    &app.config.theme_provider(),
                ),
                helpers::sub_menu_container(
                    column![
                        text("Open Ports").size(16),
                        Column::with_children(ports).spacing(5)
                    ]
                    .spacing(10),
                    &app.config.theme_provider(),
                ),
            ]
            .spacing(15),
            &app.config.theme_provider(),
        );

        column![progress_container, results_container].spacing(20)
    }
}

pub fn subscription() -> Subscription<Msg> {
    iced::Subscription::run_with_id(
        std::any::TypeId::of::<()>(),
        futures::stream::once(async {
            let rx = create_network_scanner().await;

            // Create a stream from the receiver
            futures::stream::unfold(rx, |mut rx| async move {
                rx.recv().await.map(|scan_msg| {
                    let msg = match scan_msg {
                        ScanMessage::Result(scanned_ip) => Msg::PingResult(scanned_ip),
                        ScanMessage::Complete => Msg::ScanComplete,
                    };
                    (msg, rx)
                })
            })
        })
        .flatten(),
    )
}

/// Extension trait for ScannedIp to provide UI element methods
pub trait ScannedIpExt {
    fn ping_elem(&self, theme_colors: net_monkey_theme::SimpleColors) -> Element<'_, Msg>;
    fn ips_elem(&self, theme_colors: net_monkey_theme::SimpleColors) -> Element<'_, Msg>;
    fn ports_elem(&self, theme_colors: net_monkey_theme::SimpleColors) -> Element<'_, Msg>;
}

impl ScannedIpExt for ScannedIp {
    fn ping_elem(&self, theme_colors: net_monkey_theme::SimpleColors) -> Element<'_, Msg> {
        // Color-code ping times: green for fast, yellow for medium, red for slow
        let ping_text = text(self.ping.to_string() + "ms").width(Fill).center();

        if self.ping < 50 {
            ping_text.style(move |_theme| iced::widget::text::Style {
                color: Some(theme_colors.success_color()),
            })
        } else if self.ping < 150 {
            ping_text.style(move |_theme| iced::widget::text::Style {
                color: Some(theme_colors.warning_color()),
            })
        } else {
            ping_text.style(move |_theme| iced::widget::text::Style {
                color: Some(theme_colors.danger_color()),
            })
        }
        .into()
    }

    fn ips_elem(&self, theme_colors: net_monkey_theme::SimpleColors) -> Element<'_, Msg> {
        text(self.ip.to_string())
            .width(Fill)
            .center()
            .style(move |_theme| iced::widget::text::Style {
                color: Some(theme_colors.text.into()),
            })
            .into()
    }

    fn ports_elem(&self, theme_colors: net_monkey_theme::SimpleColors) -> Element<'_, Msg> {
        let ports_text = text(self.ports_to_string()).width(Fill).center();

        if self.ports.is_empty() {
            ports_text.style(move |_theme| iced::widget::text::Style {
                color: Some(theme_colors.danger_color()),
            })
        } else {
            ports_text.style(move |_theme| iced::widget::text::Style {
                color: Some(theme_colors.text_color()),
            })
        }
        .into()
    }
}
