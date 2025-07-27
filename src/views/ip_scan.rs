use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;

use futures::StreamExt;
use iced::widget::Column;
use iced::widget::{button, column, progress_bar, row, stack, text};
use iced::{Element, Fill, Subscription};

use crate::views::settings::IpScannerApp;
use crate::{Msg, hero_image};

pub fn view(app: &IpScannerApp) -> Column<'_, Msg> {
    match app.ips.is_empty() {
        true => {
            column!(
                stack!(
                    hero_image(),
                    button(
                        text("Scan Network")
                            .width(Fill)
                            .center()
                            .size(20)
                            .color([0.7, 0.7, 0.7])
                    )
                    .style(button::primary)
                    .on_press(Msg::BeginScan)
                    .width(Fill)
                    .padding(8),
                ),
                text("If I were a grease monkey\nwhy would I need this net?")
                    .width(Fill)
                    .center()
                    .size(30)
                    .color([0.7, 0.7, 0.7])
            )
        }
        false => {
            let ping = app.ips.iter().map(ScannedIp::ping_elem);
            let ips = app.ips.iter().map(ScannedIp::ips_elem);
            let ports = app.ips.iter().map(ScannedIp::ports_elem);
            column!(
                progress_bar(0.0..=255.0, app.scan_progress as f32),
                row![
                    Column::with_children(ping).spacing(10),
                    Column::with_children(ips).spacing(10),
                    Column::with_children(ports).spacing(10),
                ]
            )
        }
    }
}

pub fn subscription() -> Subscription<Msg> {
    iced::Subscription::run_with_id(
        std::any::TypeId::of::<()>(),
        futures::stream::once(async {
            let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

            // Spawn the scanning task
            tokio::spawn(async move {
                let client = surge_ping::Client::new(&surge_ping::Config::default()).unwrap();

                let mut ping_futures = Vec::new();
                for n in 0..=255 {
                    let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, n));
                    let client = client.clone();
                    let tx = tx.clone();

                    let ping_future = async move {
                        let mut pinger = client.pinger(ip, surge_ping::PingIdentifier(0)).await;
                        match pinger
                            .timeout(Duration::from_millis(5000)) // 5 second timeout
                            .ping((n as u16).into(), &[])
                            .await
                        {
                            Ok((_, duration)) => {
                                println!("Ping successful for {ip}: {duration:?}");
                                let _ = tx.send(Msg::PingResult(ScannedIp {
                                    ping: duration.as_millis(),
                                    ip,
                                    alive: true,
                                    ports: Vec::new(),
                                }));
                            }
                            Err(_) => {
                                println!("Ping failed for {ip}");
                            }
                        }
                    };
                    ping_futures.push(ping_future);
                }

                // Wait for all pings
                futures::future::join_all(ping_futures).await;
                let _ = tx.send(Msg::ScanComplete);
            });

            // Create a stream from the receiver
            futures::stream::unfold(
                rx,
                |mut rx| async move { rx.recv().await.map(|msg| (msg, rx)) },
            )
        })
        .flatten(),
    )
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct ScannedIp {
    alive: bool,
    ip: IpAddr,
    ping: u128,
    ports: Vec<u16>,
}
impl ScannedIp {
    fn ping_elem<'a, Message, Theme, Renderer>(&self) -> Element<'a, Message, Theme, Renderer>
    where
        Theme: iced::widget::text::Catalog + 'a,
        Renderer: iced_core::text::Renderer + 'a,
    {
        text(self.ping.to_string() + "ms")
            .width(Fill)
            .center()
            .into()
    }
    fn ips_elem<'a, Message, Theme, Renderer>(&self) -> Element<'a, Message, Theme, Renderer>
    where
        Theme: iced::widget::text::Catalog + 'a,
        Renderer: iced_core::text::Renderer + 'a,
    {
        text(self.ip.to_string()).width(Fill).center().into()
    }
    fn ports_elem<'a, Message, Theme, Renderer>(&self) -> Element<'a, Message, Theme, Renderer>
    where
        Theme: iced::widget::text::Catalog + 'a,
        Renderer: iced_core::text::Renderer + 'a,
    {
        text(self.ports_to_string()).width(Fill).center().into()
    }
    pub fn ports_to_string(&self) -> String {
        match self.ports.is_empty() {
            true => String::from("<none>"),
            false => self
                .ports
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(", "),
        }
    }
}
