# Agents.md - Net Monkey Development Guide

## Project Overview

**Net Monkey** is a cross-platform network monitoring and scanning application built with Rust and the Iced GUI framework. The project aims to provide network administrators and developers with a comprehensive toolkit for network discovery, monitoring, and testing across Windows, Linux, and macOS platforms.

### Core Goals

1. **Cross-Platform Network Tools**: Provide consistent network scanning and monitoring capabilities across all major operating systems
2. **Modern GUI Experience**: Deliver a responsive, accessible user interface using the Iced framework
3. **Modular Architecture**: Maintain clean separation of concerns through a multi-crate workspace structure
4. **Advanced Theming**: Support both custom themes and COSMIC desktop integration
5. **Performance & Reliability**: Ensure fast, accurate network operations with robust error handling

### Key Features

- **Network Adapter Discovery**: Automatic detection of network interfaces with IP and MAC address information
- **IP Range Scanning**: Comprehensive subnet scanning with ping capabilities
- **TCP/UDP Tools**: Client and server functionality for network testing
- **Custom Theming**: Advanced color customization with COSMIC desktop integration
- **Cross-Platform Compatibility**: Native support for Windows, Linux, and macOS

## Framework & Technology Stack

### Primary Framework
- **Rust 2024 Edition**: Systems programming language for performance and safety
- **Iced 0.13+**: Modern GUI framework for cross-platform applications
- **Tokio**: Async runtime for concurrent network operations

### Key Dependencies
- **surge-ping**: Cross-platform ping implementation
- **if-addrs**: Network interface enumeration
- **mac_address2**: Hardware address detection
- **serde**: Serialization for configuration management
- **futures**: Async stream processing

### Optional Integrations
- **libcosmic**: COSMIC desktop environment integration (Linux)
- **image**: Asset and icon handling
- **uuid**: Unique identifier generation

## Workspace Structure

Net Monkey uses a multi-crate Rust workspace with clear separation of concerns:

```
net_monkey/
├── core/                    # Core networking functionality
│   ├── adaptor.rs          # Network adapter discovery
│   ├── scanner.rs          # IP scanning and ping operations
│   └── tasks.rs            # Task management utilities
├── theme/                   # Advanced theming system
│   ├── colors.rs           # Color palettes and theme definitions
│   ├── cosmic_integration.rs # COSMIC desktop integration
│   └── lib.rs              # Theme management API
├── components/             # Reusable UI components
│   ├── dropdown.rs         # Text input with dropdown suggestions
│   ├── label_with_hint.rs  # Labels with help tooltips
│   ├── subnet_slider.rs    # Subnet mask selection slider
│   └── text_input_with_hint.rs # Enhanced text input fields
├── app/                    # Main application binary
│   ├── main.rs            # Application entry point
│   ├── assets/            # Icons, images, and resources
│   └── views/             # Application views and screens
└── examples/               # Component demonstrations
    └── src/               # Individual component examples
```

### Crate Responsibilities

- **`net_monkey_core`**: Platform-specific network operations, adapter discovery, scanning logic
- **`net_monkey_theme`**: Theme management, color science, COSMIC integration
- **`net_monkey_components`**: Reusable UI widgets with consistent theming
- **`net_monkey`**: Main application, view logic, user interactions
- **`net_monkey_examples`**: Standalone component demonstrations

## AI Coding Guidelines

### Architecture Principles

1. **Workspace-Aware Development**
   - Always consider which crate a feature belongs in
   - Maintain clear boundaries between core logic, theming, components, and application, if unsure stop and ask
   - Use workspace dependencies to ensure version consistency

2. **Cross-Platform First**
   - Write platform-agnostic code by default and put this code in the `net_monkey_core` crate
   - Use conditional compilation (`#[cfg(target_os = "...")]`) for platform-specific implementations
   - Test assumptions about file paths, network interfaces, and system calls

3. **Theme Integration**
   - All UI components must support the theme system
   - Use `net_monkey_theme::helpers` for consistent styling
   - Support both standard and COSMIC theme modes
   - Never hardcode colors - always derive from theme

### Code Style Standards

#### Rust Conventions
```rust
// Use explicit imports from workspace crates
use net_monkey_core::{NetworkAdapter, get_network_adapters};
use net_monkey_theme::{AdaptiveThemeManager, helpers};
use net_monkey_components::TextInputWithHint;

// Prefer explicit error handling
match network_operation() {
    Ok(result) => handle_success(result),
    Err(e) => {
        eprintln!("Network operation failed: {}", e);
        return Err(e.into());
    }
}

// Use descriptive variable names for network concepts
let subnet_mask = 24; // Not just 'mask'
let network_adapter = selected_adapter; // Not just 'adapter'
let ping_timeout_ms = 1000; // Include units in variable names
```

#### Async Patterns
```rust
// Use tokio for async network operations
use tokio::time::{sleep, Duration};

async fn scan_ip_range(start_ip: IpAddr, end_ip: IpAddr) -> Result<Vec<ScannedIp>, ScanError> {
    let mut results = Vec::new();
    let mut tasks = Vec::new();

    // Batch concurrent operations
    for ip in ip_range(start_ip, end_ip) {
        let task = tokio::spawn(async move {
            ping_host(ip, Duration::from_millis(1000)).await
        });
        tasks.push(task);

        // Limit concurrency to avoid overwhelming the network
        if tasks.len() >= 50 {
            for task in tasks.drain(..) {
                if let Ok(result) = task.await {
                    results.extend(result);
                }
            }
        }
    }

    Ok(results)
}
```

#### UI Component Patterns
```rust
// Always use the theme system for styling
use net_monkey_theme::{NetMonkeyColors, helpers};

pub fn network_status_view<'a>(
    adapter: &NetworkAdapter,
    theme: NetMonkeyTheme,
) -> Element<'a, Message> {
    let theme_colors = theme.colors();

    let status_color = if adapter.is_connected {
        theme_colors.success
    } else {
        theme_colors.error
    };

    let content = column![
        text(&adapter.name).color(theme_colors.text),
        text(&adapter.ip_address).size(14).color(theme_colors.text_secondary),
        container(text("Connected").color(status_color))
            .style(helpers::status_container_style(theme))
    ].spacing(8);

    helpers::card_container(content, theme).into()
}
```

### Error Handling Patterns

1. **Network Operations**
   ```rust
   // Use specific error types for network operations
   #[derive(Debug, thiserror::Error)]
   pub enum NetworkError {
       #[error("Network adapter not found: {name}")]
       AdapterNotFound { name: String },
       #[error("Ping timeout after {timeout_ms}ms")]
       PingTimeout { timeout_ms: u64 },
       #[error("Invalid IP address format: {input}")]
       InvalidIpAddress { input: String },
   }
   ```

2. **UI Error Display**
   ```rust
   // Provide user-friendly error messages
   fn handle_scan_error(error: NetworkError) -> Element<Message> {
       let (title, description) = match error {
           NetworkError::AdapterNotFound { name } => (
               "Network Adapter Error",
               format!("Could not find network adapter '{}'. Please check your network settings.", name)
           ),
           NetworkError::PingTimeout { timeout_ms } => (
               "Network Timeout",
               format!("Network requests timed out after {}ms. The target may be unreachable.", timeout_ms)
           ),
           _ => ("Network Error", "An unexpected network error occurred.".to_string()),
       };

       helpers::error_dialog(title, &description, theme)
   }
   ```

### Testing Guidelines

1. **Unit Tests for Core Logic**
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[tokio::test]
       async fn test_ip_range_generation() {
           let start = "192.168.1.1".parse().unwrap();
           let end = "192.168.1.5".parse().unwrap();
           let range: Vec<_> = ip_range(start, end).collect();
           assert_eq!(range.len(), 5);
       }

       #[test]
       fn test_subnet_mask_calculation() {
           assert_eq!(subnet_to_range("192.168.1.0/24").len(), 256);
           assert_eq!(subnet_to_range("10.0.0.0/16").len(), 65536);
       }
   }
   ```

2. **Integration Tests for Components**
   ```rust
   // Use the examples crate for component testing
   cargo run -p net_monkey_examples --bin component_test
   ```

### Performance Considerations

1. **Network Operations**
   - Limit concurrent ping operations (recommended: 50-100 concurrent)
   - Use appropriate timeouts (1-5 seconds for pings)
   - Cache network adapter information
   - Implement proper backoff for failed operations

2. **UI Responsiveness**
   - Use `tokio::spawn` for long-running network operations
   - Update UI progressively during scans
   - Provide cancel functionality for long operations
   - Use lazy loading for large result sets

### Platform-Specific Guidelines

#### Windows
```rust
#[cfg(target_os = "windows")]
mod windows_impl {
    // Use PowerShell for network adapter detection
    use std::process::Command;

    pub fn get_adapters_powershell() -> Result<Vec<NetworkAdapter>, NetworkError> {
        let output = Command::new("powershell")
            .args(["-Command", "Get-NetAdapter | ConvertTo-Json"])
            .output()?;
        // Parse JSON output...
    }
}
```

#### Linux
```rust
#[cfg(target_os = "linux")]
mod linux_impl {
    // Read from /sys/class/net for adapter information
    use std::fs;

    pub fn get_adapter_mac(interface: &str) -> Result<String, std::io::Error> {
        let path = format!("/sys/class/net/{}/address", interface);
        fs::read_to_string(path).map(|s| s.trim().to_string())
    }
}
```

#### macOS
```rust
#[cfg(target_os = "macos")]
mod macos_impl {
    // Use ifconfig command for adapter detection
    use std::process::Command;

    pub fn get_adapters_ifconfig() -> Result<Vec<NetworkAdapter>, NetworkError> {
        let output = Command::new("ifconfig").output()?;
        // Parse ifconfig output...
    }
}
```

### Theme Development Guidelines

1. **Color Science**
   - Use OKLCH color space when possible
   - Ensure WCAG AA compliance for contrast ratios
   - Provide both light and dark theme variants
   - Test themes with color vision deficiency simulators

2. **COSMIC Integration**
   ```rust
   #[cfg(feature = "cosmic")]
   use net_monkey_theme::with_cosmic;

   fn apply_theme(state: &AppState) -> Theme {
       with_cosmic!(
           {
               // COSMIC-specific theming
               let cosmic_theme = CosmicTheme::system();
               cosmic_theme.to_iced_theme()
           },
           {
               // Fallback theming
               state.custom_theme.to_iced_theme()
           }
       )
   }
   ```

### Documentation Requirements

1. **Public APIs**: Document all public functions with examples
2. **Platform Differences**: Clearly document platform-specific behavior
3. **Error Conditions**: Document when and why functions can fail
4. **Performance**: Document complexity and resource usage
5. **Examples**: Provide working examples for complex features

### Build and Deployment

1. **Feature Flags**
   ```bash
   # Standard build (all platforms)
   cargo build --release

   # COSMIC integration (Linux)
   cargo build --release --features cosmic

   # Development with all features
   cargo build --all-features
   ```

2. **Testing Matrix**
   - Test on Windows, Linux, and macOS
   - Test with and without COSMIC feature
   - Verify network operations on different network configurations
   - Test theme switching and color customization

### Common Pitfalls to Avoid

1. **Network Operations**
   - Don't assume network interfaces have consistent naming across platforms
   - Don't hardcode network timeouts - make them configurable
   - Don't ignore network adapter state changes
   - Don't perform blocking network operations on the UI thread

2. **Cross-Platform Development**
   - Don't use platform-specific paths without proper abstraction
   - Don't assume availability of specific system commands
   - Don't hardcode line endings or file separators
   - Don't use platform-specific network interface names

3. **Theme System**
   - Don't bypass the theme system for any UI styling
   - Don't assume theme colors will remain constant
   - Don't hardcode color values anywhere in the UI
   - Don't forget to test both light and dark theme variants

### Performance Targets

- **Application Startup**: < 2 seconds on modern hardware
- **Network Adapter Discovery**: < 1 second
- **IP Scan (Class C subnet)**: < 30 seconds with 50 concurrent pings
- **Theme Switching**: < 500ms for UI update
- **Memory Usage**: < 50MB for typical usage

By following these guidelines, you'll contribute to a maintainable, performant, and user-friendly network monitoring application that works consistently across all supported platforms.
