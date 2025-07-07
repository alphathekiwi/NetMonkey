# IP Scanner

A cross-platform network scanner application built with Rust and Iced GUI framework.

## Features

- **Cross-Platform Network Adapter Detection**: Works on Windows, Linux, and macOS
- **Network Interface Information**: Retrieves name, IP address, and MAC address for each active network adapter
- **GUI Interface**: Built with Iced for a modern, responsive user interface
- **Multiple Network Tools**: IP scanning, TCP/UDP client/server functionality (coming soon)

## Cross-Platform Network Adapter Support

This application includes robust cross-platform network adapter detection that works consistently across different operating systems:

### Supported Platforms

- **Windows**: Uses PowerShell and WMI queries for accurate adapter information
- **Linux**: Reads from `/sys/class/net/{interface}/address` for MAC addresses
- **macOS**: Uses `ifconfig` command to retrieve network adapter details

### Network Adapter Information

For each active network adapter, the application provides:
- **Name**: Interface name or identifier
- **IP Address**: Currently assigned IP address (IPv4 or IPv6)
- **MAC Address**: Hardware MAC address of the network adapter

## Building and Running

### Prerequisites

- Rust (latest stable version)
- Cargo package manager

### Dependencies

The application uses the following key dependencies:
- `iced` - GUI framework
- `if-addrs` - Cross-platform network interface enumeration
- `tokio` - Async runtime
- `serde` - Serialization support

### Building

```bash
cargo build --release
```

### Running

```bash
cargo run
```

### Testing

Run the network adapter detection test:

```bash
cargo test network_adapters -- --nocapture
```

This will display all detected network adapters with their information.

## Usage Example

The network adapter functionality can be used programmatically:

```rust
let adapters = get_network_adapters();
for adapter in adapters {
    println!("Interface: {} - IP: {} - MAC: {}", 
             adapter.name, adapter.ip_address, adapter.mac_address);
}
```

## Platform-Specific Implementation Details

### Windows
- Uses PowerShell `Get-NetAdapter` cmdlet for primary detection
- Falls back to WMI queries for compatibility
- Handles Windows GUID-based interface names

### Linux
- Reads MAC addresses from `/sys/class/net/{interface}/address`
- Uses standard Linux network interface naming

### macOS
- Uses `ifconfig` command to parse network interface information
- Extracts MAC addresses from `ether` lines in ifconfig output

## Architecture

The application is structured with:
- **Cross-platform abstraction**: Single API that works on all supported platforms
- **Platform-specific implementations**: Optimized code for each operating system
- **Error handling**: Graceful degradation when network information is unavailable
- **Modern GUI**: Iced-based interface with tabbed navigation

## Future Features

- IP range scanning
- Port scanning
- TCP/UDP client and server tools
- Network discovery and mapping
- Export functionality for scan results

## Contributing

Contributions are welcome! Please ensure that any network-related functionality maintains cross-platform compatibility.

## License

This project is open source. Please check the license file for details.