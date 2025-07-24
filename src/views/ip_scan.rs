use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;

use futures::StreamExt;
use iced::widget::Column;
use iced::widget::{button, column, text};
use iced::{Element, Fill, Subscription};

use crate::{Msg, hero_image};

pub fn view(ips: &[ScannedIp]) -> Column<'_, Msg> {
    match ips.is_empty() {
        true => {
            column!(
                button(hero_image())
                    .style(button::primary)
                    .on_press(Msg::BeginScan)
                    .width(Fill)
                    .padding(8),
                text("If I were a grease monkey\nwhy would I need this net?")
                    .width(Fill)
                    .center()
                    .size(30)
                    .color([0.7, 0.7, 0.7])
            )
        }
        false => {
            let ips = ips.iter().map(|s| s.into());
            Column::with_children(ips).spacing(10)
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
    ip: IpAddr,
    alive: bool,
    ports: Vec<u16>,
}
impl<'a, Message, Theme, Renderer> From<&ScannedIp> for Element<'a, Message, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced_core::text::Renderer + 'a,
{
    fn from(scan: &ScannedIp) -> Element<'a, Message, Theme, Renderer> {
        text(scan.ip.to_string()).width(Fill).center().into()
    }
}
