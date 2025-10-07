use std::net::IpAddr;
use std::time::Duration;
use surge_ping::{Client, Config, PingIdentifier, PingSequence};

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    // create a tokio channel to send data from async to sync land
    rt.block_on(async {
        if let Err(err) = failable_main().await {
            println!("Error: {err}");
            std::process::exit(1);
        }
    });
}

async fn failable_main() -> Result<(), Box<dyn std::error::Error>> {
    let client_v4 = Client::new(&Config::default())?;
    let mut handles = Vec::new();
    let (tx, mut rx) = tokio::sync::mpsc::channel::<ScanResult>(100);
    tokio::spawn(async move {
        while let Some(result) = rx.recv().await {
            let ScanResult { ip, ping, is_alive } = result;
            if is_alive {
                println!("{ip} ping time: {} ms", ping.unwrap().as_millis());
            }
        }
    });
    for ip in 1..255 {
        let ip_addr: IpAddr = format!("192.168.1.{ip}").parse().unwrap();
        let client = client_v4.clone();
        let tx = tx.clone();
        handles.push(tokio::spawn(async move {
            let mut pinger = client.pinger(ip_addr, PingIdentifier(ip)).await;
            // pinger.timeout(Duration::from_millis(8000));
            // pinger.send_ping(seq, payload)
            let res = match pinger.ping(PingSequence(ip), &[]).await {
                Ok((_, duration)) => ScanResult {
                    ip: ip_addr,
                    is_alive: true,
                    ping: Some(duration),
                },
                Err(_) => ScanResult {
                    ip: ip_addr,
                    is_alive: false,
                    ping: None,
                },
            };
            tx.send(res).await.ok();
        }));
    }

    for handle in handles {
        let _res = handle.await;
    }

    Ok(())
}

struct ScanResult {
    ip: IpAddr,
    ping: Option<Duration>,
    is_alive: bool,
}
