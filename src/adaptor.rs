use std::net::IpAddr;

use if_addrs::get_if_addrs;

#[derive(Debug, Clone)]
struct NetworkAdapter {
    name: String,
    ip_address: String,
    mac_address: String,
}

fn get_network_adapters() -> Vec<NetworkAdapter> {
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
        "Get-NetAdapter | Where-Object {{ $_.InterfaceGuid -eq '{}' }} | Select-Object -ExpandProperty MacAddress",
        interface_name
    );

    let output = Command::new("powershell")
        .args(["-Command", &ps_script])
        .output();

    match output {
        Ok(output) => {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let mac = output_str.trim();
            if !mac.is_empty() && mac != "MacAddress" {
                return mac.to_string();
            }
        }
        Err(_) => {}
    }

    let wmi_query = format!(
        "wmic path win32_networkadapter where \"GUID='{}' and NetEnabled=true\" get MACAddress /format:list",
        interface_name
    );

    let output = Command::new("cmd").args(["/C", &wmi_query]).output();

    match output {
        Ok(output) => {
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
        Err(_) => {}
    }

    "Unknown".to_string()
}

#[cfg(target_os = "linux")]
fn get_mac_address_linux(interface_name: &str) -> String {
    use std::fs;

    let path = format!("/sys/class/net/{}/address", interface_name);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn network_adapters() {
        let adapters = get_network_adapters();
        println!("=== Network Adapters (Cross-Platform) ===");
        println!("Found {} active network adapter(s):", adapters.len());
        println!();

        for (index, adapter) in adapters.iter().enumerate() {
            println!("Adapter #{}", index + 1);
            println!("  Name: {}", adapter.name);
            println!("  IP Address: {}", adapter.ip_address);
            println!("  MAC Address: {}", adapter.mac_address);
            println!();
        }

        assert!(
            !adapters.is_empty(),
            "Should find at least one network adapter"
        );

        for adapter in &adapters {
            assert!(!adapter.name.is_empty(), "Adapter name should not be empty");
            assert!(
                !adapter.ip_address.is_empty(),
                "IP address should not be empty"
            );
            assert!(
                !adapter.mac_address.is_empty(),
                "MAC address field should not be empty"
            );
        }
        // assert!(false);
    }

    #[test]
    fn network_adapter_struct_functionality() {
        let adapter = NetworkAdapter {
            name: "test_adapter".to_string(),
            ip_address: "192.168.1.100".to_string(),
            mac_address: "00:11:22:33:44:55".to_string(),
        };

        let cloned_adapter = adapter.clone();
        assert_eq!(adapter.name, cloned_adapter.name);
        assert_eq!(adapter.ip_address, cloned_adapter.ip_address);
        assert_eq!(adapter.mac_address, cloned_adapter.mac_address);

        let debug_output = format!("{adapter:?}");
        assert!(debug_output.contains("test_adapter"));
        assert!(debug_output.contains("192.168.1.100"));
        assert!(debug_output.contains("00:11:22:33:44:55"));
        // assert!(false);
    }
}
