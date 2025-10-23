use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;

/// Result of scanning a single IP address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannedIp {
    pub alive: bool,
    pub ip: IpAddr,
    pub ping: u128,
    pub ports: Vec<u16>,
}

impl ScannedIp {
    /// Create a new ScannedIp result
    pub fn new(ip: IpAddr, alive: bool, ping: u128, ports: Vec<u16>) -> Self {
        Self {
            alive,
            ip,
            ping,
            ports,
        }
    }

    /// Convert ports vector to display string
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

/// Scan a network range for alive hosts
///
/// This function performs ping scans on IP addresses in the range 192.168.1.0 to 192.168.1.255
/// and calls the provided callback for each successful ping result and when scanning completes.
///
/// # Arguments
/// * `result_callback` - Called for each successful ping with ScannedIp result
/// * `complete_callback` - Called when scanning is complete
///
/// # Example
/// ```rust,no_run
/// use net_monkey_core::scan_network_async;
///
/// tokio::spawn(async {
///     scan_network_async(
///         |scanned_ip| {
///             println!("Found host: {:?}", scanned_ip);
///         },
///         || {
///             println!("Scan complete!");
///         }
///     ).await;
/// });
/// ```
pub async fn scan_network_async<F, G>(
    result_callback: F,
    complete_callback: G,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn(ScannedIp) + Send + Sync + 'static,
    G: Fn() + Send + Sync + 'static,
{
    let client = surge_ping::Client::new(&surge_ping::Config::default())?;

    let mut ping_futures = Vec::new();
    for n in 0..=255 {
        let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, n));
        let client = client.clone();
        let result_callback = &result_callback;

        let ping_future = async move {
            let mut pinger = client.pinger(ip, surge_ping::PingIdentifier(0)).await;
            match pinger
                .timeout(Duration::from_millis(5000)) // 5 second timeout
                .ping((n as u16).into(), &[])
                .await
            {
                Ok((_, duration)) => {
                    println!("Ping successful for {ip}: {duration:?}");
                    let scanned_ip = ScannedIp::new(ip, true, duration.as_millis(), Vec::new());
                    result_callback(scanned_ip);
                }
                Err(_) => {
                    println!("Ping failed for {ip}");
                }
            }
        };
        ping_futures.push(ping_future);
    }

    // Wait for all pings to complete
    futures::future::join_all(ping_futures).await;

    // Signal completion
    complete_callback();

    Ok(())
}

/// Create a tokio channel-based network scanner
///
/// This function returns a channel receiver that yields scan results as they come in.
/// It's designed to work with async streaming systems like Iced subscriptions.
///
/// # Returns
/// * `tokio::sync::mpsc::UnboundedReceiver<ScanMessage>` - Channel receiver for scan results
///
/// # Example
/// ```rust,no_run
/// use net_monkey_core::create_network_scanner;
///
/// let mut rx = create_network_scanner().await;
/// while let Some(message) = rx.recv().await {
///     match message {
///         ScanMessage::Result(scanned_ip) => {
///             println!("Found: {:?}", scanned_ip);
///         }
///         ScanMessage::Complete => {
///             println!("Scan finished");
///             break;
///         }
///     }
/// }
/// ```
pub async fn create_network_scanner(
    ip: IpAddr,
    mask: u8,
) -> tokio::sync::mpsc::UnboundedReceiver<ScanMessage> {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

    // Spawn the scanning task
    tokio::spawn(async move {
        let client = surge_ping::Client::new(&surge_ping::Config::default()).unwrap();

        let mut ping_futures = Vec::new();
        let ip_range = 0xffffffff_u32 >> mask;
        for n in 0..=ip_range {
            let ip = IpAddr::V4();
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
                        let scanned_ip = ScannedIp::new(ip, true, duration.as_millis(), Vec::new());
                        let _ = tx.send(ScanMessage::Result(scanned_ip));
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
        let _ = tx.send(ScanMessage::Complete);
    });

    rx
}

/// Messages sent by the network scanner
#[derive(Debug, Clone)]
pub enum ScanMessage {
    /// A scan result for a single IP
    Result(ScannedIp),
    /// Scanning is complete
    Complete,
}
