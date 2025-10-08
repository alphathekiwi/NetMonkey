use std::fs::read_to_string;
use std::net::IpAddr;

use crate::Msg;

use iced::Alignment::Center;
use iced::Element;
use iced::Length::Fill;
use iced::widget::{column, scrollable, text, text_input};
use iced_widget::{button, horizontal_rule, pick_list, row};
use net_monkey_components::{LabelWithHint, SubnetSlider, TextInputDropdown};
use net_monkey_core::{NetworkAdapter, ScannedIp};
use net_monkey_theme::{NetMonkeyTheme, ThemeManager};
use serde::{Deserialize, Serialize};

pub fn view<'a>(app: &'a IpScannerApp) -> Element<'a, Msg> {
    let items = app.adaptors.clone();
    println!("{items:?}");
    let ip_sel: TextInputDropdown<_, _, Msg, iced::Theme> = TextInputDropdown::new(
        items,
        app.config.starting_ip.to_string(),
        |s| Msg::Config(ChangeConfig::StartingIp(s)),
        |s| Msg::Config(ChangeConfig::StartingIp(s.ip_address)),
    )
    .text_size(24);
    let subnet_slider = SubnetSlider::new(app.config.subnet_mask, Msg::subnet_mask)
        .text_size(24.0)
        .height(45.0)
        .into_element();

    // Create theme selector dropdown
    let theme_options = NetMonkeyTheme::all();

    scrollable(
        column![
            text("Network Configuration").size(22),
            horizontal_rule(2),
            text("Starting IP").size(18),
            iced::Element::from(ip_sel),
            text("Subnet Mask").size(18),
            subnet_slider,
            LabelWithHint::new(
                "Ports List",
                "Comma-separated list of ports to scan (e.g., 80, 443, 22)"
            )
            .text_size(18.0)
            .theme(app.config.theme.clone())
            .into_element(),
            text_input("Ports List", &app.config.ports_to_string())
                .on_input(|s| Msg::Config(ChangeConfig::Ports(s)))
                .size(24),
            text("Appearance").size(22),
            horizontal_rule(2),
            LabelWithHint::new("Theme", app.config.theme.description())
                .text_size(18.0)
                .theme(app.config.theme.clone())
                .into_element(),
            row![
                pick_list(theme_options, Some(app.config.theme.clone()), |theme| {
                    Msg::Config(ChangeConfig::Theme(theme))
                })
                .text_size(24)
                .width(Fill),
                button(text(String::from("Edit Theme")).width(Fill).center())
                    .on_press(Msg::TabChanged(ModeTab::ThemeEdit))
                    .width(Fill)
                    .padding(8),
            ]
            .spacing(8),
        ]
        .align_x(Center)
        .spacing(12)
        .padding(20),
    )
    .height(Fill)
    .direction(iced::widget::scrollable::Direction::Vertical(
        iced::widget::scrollable::Scrollbar::new()
            .width(8)
            .scroller_width(8),
    ))
    .into()
}

#[derive(Debug)]
pub struct IpScannerApp {
    pub tab: ModeTab,
    // IP Scanner
    pub ips: Vec<ScannedIp>,
    pub scan_progress: u8,
    // TCP Connection
    pub tcp_ip_port: String,
    pub tcp_ip_address: String,
    pub tcp_connection: Option<IpAddr>,
    pub tcp_history: Vec<String>,
    // UDP Connection
    pub udp_ip_port: String,
    pub udp_ip_address: String,
    pub udp_connection: Option<IpAddr>,
    pub udp_history: Vec<String>,
    // Settings
    pub adaptors: Vec<NetworkAdapter>,
    pub config: AppConfig,
}
impl IpScannerApp {
    pub fn loaded(&mut self, c: AppConfig, a: Vec<NetworkAdapter>) {
        // Ensure default themes exist and cleanup temporary themes
        ThemeManager::ensure_default_themes();
        ThemeManager::cleanup_temporary_themes();
        self.config = c;
        self.adaptors = a;
    }
}

// let state = SettingsState {
//     state: combo_box::State::new(adaptors.into()),
//     selected: None,
// };
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppConfig {
    pub starting_ip: String,
    pub subnet_mask: u8,
    pub ports: Vec<u16>,
    pub forced_ip_mode: ForcedIPMode,
    pub theme: NetMonkeyTheme,
}
impl Default for AppConfig {
    fn default() -> Self {
        Self {
            starting_ip: String::from("192.168.1.1"),
            subnet_mask: 24,
            ports: vec![80, 443],
            forced_ip_mode: ForcedIPMode::Any,
            theme: NetMonkeyTheme::Loaded("Dark".to_string()),
        }
    }
}
impl AppConfig {
    pub fn ports_to_string(&self) -> String {
        self.ports
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(", ")
    }
    fn left_shift(&self, value: u8) -> u8 {
        match self.subnet_mask < value {
            true if value - self.subnet_mask > 7 => 0,
            true => 255 << (value - self.subnet_mask).min(8),
            false => 255,
        }
    }
    pub fn subnet_mask_long(&self) -> String {
        format!(
            "{}.{}.{}.{}",
            self.left_shift(8u8),
            self.left_shift(16u8),
            self.left_shift(24u8),
            self.left_shift(32u8)
        )
    }
    pub fn update(&mut self, change: ChangeConfig) {
        match change {
            ChangeConfig::StartingIp(ip) => self.starting_ip = ip,
            ChangeConfig::SubnetMask(mask) => self.subnet_mask = mask.parse().unwrap_or_default(),
            ChangeConfig::Ports(ports) => {
                self.ports = ports.split(',').filter_map(|p| p.parse().ok()).collect()
            }
            ChangeConfig::ForcedIPMode(mode) => self.forced_ip_mode = mode.into(),
            ChangeConfig::Theme(theme) => {
                self.theme = theme;
            }
            ChangeConfig::ColorChange(color_type, hex_value) => {
                // Color changes now modify the current theme directly
                if let Some(color) = hex_to_color(&hex_value) {
                    // Create a modified theme based on current theme
                    let mut current_colors = self.theme.colors();
                    let serializable_color = color.into();

                    match color_type {
                        ColorType::Background => current_colors.background = serializable_color,
                        ColorType::Menu => current_colors.menu = serializable_color,
                        ColorType::SubMenu => current_colors.sub_menu = serializable_color,
                        ColorType::Text => current_colors.text = serializable_color,
                        ColorType::TextSecondary => {
                            current_colors.text_secondary = serializable_color
                        }
                        ColorType::Primary => current_colors.primary = serializable_color,
                        ColorType::Success => current_colors.success = serializable_color,
                        ColorType::Warning => current_colors.warning = serializable_color,
                        ColorType::Danger => current_colors.danger = serializable_color,
                        ColorType::Border => current_colors.border = serializable_color,
                        ColorType::BorderFocused => {
                            current_colors.border_focused = serializable_color
                        }
                        ColorType::BorderHover => current_colors.border_hover = serializable_color,
                        ColorType::BorderDisabled => {
                            current_colors.border_disabled = serializable_color
                        }
                        ColorType::Active => current_colors.active = serializable_color,
                        ColorType::Hover => current_colors.hover = serializable_color,
                    }

                    // Save the modified theme as a temporary theme
                    use net_monkey_theme::{ThemeDefinition, ThemeManager};
                    use std::time::{SystemTime, UNIX_EPOCH};

                    let timestamp = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();

                    let temp_theme = ThemeDefinition {
                        name: format!("editing_{timestamp}"),
                        description: "Temporary theme being edited".to_string(),
                        colors: current_colors,
                        is_dark: current_colors.background.r < 0.5,
                    };

                    if ThemeManager::save_theme(&temp_theme).is_ok() {
                        self.theme = NetMonkeyTheme::Loaded(temp_theme.name);
                    }
                }
            }
        }
    }
    pub fn load() -> Option<Self> {
        let config_path = Self::config_file_path();
        serde_json::from_str(&read_to_string(config_path).ok()?).ok()
    }
    pub fn save(&self) -> anyhow::Result<()> {
        let config_path = Self::config_file_path();
        let config_dir = std::path::Path::new(&config_path).parent().unwrap();
        std::fs::create_dir_all(config_dir)?;
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(config_path, json)?;
        Ok(())
    }

    /// Get the config file path based on build mode
    fn config_file_path() -> String {
        #[cfg(debug_assertions)]
        {
            // In debug mode, find the workspace root and use app/data/config.json
            if let Ok(current_dir) = std::env::current_dir() {
                let mut path = current_dir;
                // Look for workspace Cargo.toml (contains [workspace]) to identify workspace root
                loop {
                    let cargo_toml = path.join("Cargo.toml");
                    if cargo_toml.exists()
                        && let Ok(content) = std::fs::read_to_string(&cargo_toml)
                        && content.contains("[workspace]")
                    {
                        break;
                    }
                    if !path.pop() {
                        // Fallback if we can't find workspace root
                        return "app/data/config.json".to_string();
                    }
                }
                path.push("app");
                path.push("data");
                path.push("config.json");
                path.to_string_lossy().to_string()
            } else {
                "app/data/config.json".to_string()
            }
        }
        #[cfg(not(debug_assertions))]
        {
            // In release mode, use current working directory
            "data/config.json".to_string()
        }
    }
}
impl Drop for AppConfig {
    fn drop(&mut self) {
        if let Err(e) = self.save() {
            eprintln!("Failed to save config: {e}");
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum ForcedIPMode {
    Any,
    V4,
    V6,
}
impl From<usize> for ForcedIPMode {
    fn from(mode: usize) -> Self {
        match mode {
            1 => ForcedIPMode::V4,
            2 => ForcedIPMode::V6,
            _ => ForcedIPMode::Any,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModeTab {
    IpScan,
    TCPclient,
    TCPserver,
    UDPclient,
    UDPserver,
    Settings,
    ThemeEdit,
}

/// Basic to string conversion for ModeTab
impl From<&ModeTab> for String {
    fn from(tab: &ModeTab) -> Self {
        match tab {
            ModeTab::IpScan => "IP Scan",
            ModeTab::TCPclient => "TCP Client",
            ModeTab::TCPserver => "TCP Server",
            ModeTab::UDPclient => "UDP Client",
            ModeTab::UDPserver => "UDP Server",
            ModeTab::Settings => "Settings",
            ModeTab::ThemeEdit => "Edit Theme",
        }
        .to_string()
    }
}

/// Default implementation for IpScannerApp
impl Default for IpScannerApp {
    fn default() -> Self {
        // Ensure default themes exist and cleanup temporary themes
        ThemeManager::ensure_default_themes();
        ThemeManager::cleanup_temporary_themes();
        let config = AppConfig::default();
        Self {
            tab: ModeTab::IpScan,
            scan_progress: 255,
            ips: Vec::new(),
            adaptors: vec![NetworkAdapter::default()],
            config,

            tcp_ip_port: String::new(),
            tcp_ip_address: String::new(),
            tcp_connection: None,
            tcp_history: Vec::new(),

            udp_connection: None,
            udp_history: Vec::new(),
            udp_ip_port: String::new(),
            udp_ip_address: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ChangeConfig {
    StartingIp(String),
    SubnetMask(String),
    Ports(String),
    ForcedIPMode(usize),
    Theme(NetMonkeyTheme),
    ColorChange(ColorType, String),
}

#[derive(Debug, Clone)]
pub enum ColorType {
    Background,
    Menu,
    SubMenu,
    Text,
    TextSecondary,
    Primary,
    Success,
    Warning,
    Danger,
    Border,
    BorderFocused,
    BorderHover,
    BorderDisabled,
    Active,
    Hover,
}

// Helper function to parse hex color
fn hex_to_color(hex: &str) -> Option<iced::Color> {
    if !hex.starts_with('#') || hex.len() != 7 {
        return None;
    }

    let hex = &hex[1..];
    if let (Ok(r), Ok(g), Ok(b)) = (
        u8::from_str_radix(&hex[0..2], 16),
        u8::from_str_radix(&hex[2..4], 16),
        u8::from_str_radix(&hex[4..6], 16),
    ) {
        Some(iced::Color::from_rgb(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
        ))
    } else {
        None
    }
}
