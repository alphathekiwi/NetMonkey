use std::{fmt::Display, net::IpAddr};

use if_addrs::get_if_addrs;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkAdapter {
    pub name: String,
    pub ip_address: String,
    pub mac_address: String,
}
impl Display for NetworkAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.ip_address, self.name)
    }
}
pub fn get_network_adapters() -> Vec<NetworkAdapter> {
    let mut adapters = Vec::new();

    match get_if_addrs() {
        Ok(interfaces) => {
            for interface in interfaces {
                if !interface.is_loopback() {
                    let ip_address = match interface.ip() {
                        IpAddr::V4(ipv4) => ipv4.to_string(),
                        IpAddr::V6(ipv6) => ipv6.to_string(),
                    };

                    let mac_address = get_mac_address_for_interface(&interface.name);
                    adapters.push(NetworkAdapter {
                        name: interface.name.clone(),
                        ip_address,
                        mac_address,
                    });
                } else {
                    println!("Skipping loopback adapter {}", interface.ip())
                }
            }
        }
        Err(e) => {
            eprintln!("Error getting network interfaces: {e}");
        }
    }

    adapters
}

fn get_mac_address_for_interface(interface_name: &str) -> String {
    #[cfg(target_os = "windows")]
    {
        get_mac_address_windows(interface_name)
    }
    #[cfg(target_os = "linux")]
    {
        get_mac_address_linux(interface_name)
    }
    #[cfg(target_os = "macos")]
    {
        get_mac_address_macos(interface_name)
    }
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        "Unsupported OS".to_string()
    }
}

#[cfg(target_os = "windows")]
fn get_mac_address_windows(interface_name: &str) -> String {
    use std::process::Command;

    let ps_script = format!(
        "Get-NetAdapter | Where-Object {{ $_.InterfaceGuid -eq '{interface_name}' }} | Select-Object -ExpandProperty MacAddress",
    );

    let output = Command::new("powershell")
        .args(["-Command", &ps_script])
        .output();

    if let Ok(output) = output {
        let output_str = String::from_utf8_lossy(&output.stdout);
        let mac = output_str.trim();
        if !mac.is_empty() && mac != "MacAddress" {
            return mac.to_string();
        }
    }

    let wmi_query = format!(
        "wmic path win32_networkadapter where \"GUID='{interface_name}' and NetEnabled=true\" get MACAddress /format:list",
    );

    let output = Command::new("cmd").args(["/C", &wmi_query]).output();

    if let Ok(output) = output {
        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines() {
            if line.starts_with("MACAddress=") {
                let mac = line.replace("MACAddress=", "").trim().to_string();
                if !mac.is_empty() {
                    return mac;
                }
            }
        }
    }

    "Unknown".to_string()
}

#[cfg(target_os = "linux")]
fn get_mac_address_linux(interface_name: &str) -> String {
    use std::fs;

    let path = format!("/sys/class/net/{interface_name}/address",);
    match fs::read_to_string(&path) {
        Ok(mac) => mac.trim().to_string(),
        Err(_) => "Unknown".to_string(),
    }
}

#[cfg(target_os = "macos")]
fn get_mac_address_macos(interface_name: &str) -> String {
    use std::process::Command;

    let output = Command::new("ifconfig").arg(interface_name).output();

    match output {
        Ok(output) => {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if line.contains("ether") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        return parts[1].to_string();
                    }
                }
            }
            "Unknown".to_string()
        }
        Err(_) => "Error".to_string(),
    }
}
