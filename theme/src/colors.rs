//! Custom theme module for the NetMonkey network monitoring application.
//!
//! This module provides custom color palettes and themes optimized for
//! network monitoring interfaces, with distinct colors for different
//! UI sections like backgrounds, menus, and sub-menus.

use std::fmt::Display;

use iced::theme::{Palette, palette::Extended};
use iced::{Color, Theme};
use std::fs;

/// Serializable color wrapper for iced::Color
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SerializableColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl From<Color> for SerializableColor {
    fn from(color: Color) -> Self {
        Self {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        }
    }
}

impl From<SerializableColor> for Color {
    fn from(color: SerializableColor) -> Self {
        Color::from_rgba(color.r, color.g, color.b, color.a)
    }
}

impl Eq for SerializableColor {}

/// Custom color palette for NetMonkey application
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct NetMonkeyColors {
    /// Main application background
    pub background: SerializableColor,
    /// Menu/navigation background
    pub menu: SerializableColor,
    /// Sub-menu background
    pub sub_menu: SerializableColor,
    /// Primary text color
    pub text: SerializableColor,
    /// Secondary/muted text color
    pub text_secondary: SerializableColor,
    /// Primary accent color (for buttons, highlights)
    pub primary: SerializableColor,
    /// Success color (for successful connections, scans)
    pub success: SerializableColor,
    /// Warning color (for timeouts, warnings)
    pub warning: SerializableColor,
    /// Error/danger color (for failed connections, errors)
    pub danger: SerializableColor,
    /// Border color for components
    pub border: SerializableColor,
    /// Border color for focused elements
    pub border_focused: SerializableColor,
    /// Border color for hovered elements
    pub border_hover: SerializableColor,
    /// Border color for disabled elements
    pub border_disabled: SerializableColor,
    /// Active/selected state color
    pub active: SerializableColor,
    /// Hover state color
    pub hover: SerializableColor,
}

impl NetMonkeyColors {
    /// Dark theme color scheme - recommended for network monitoring
    pub const DARK: Self = Self {
        background: SerializableColor {
            r: 0.08,
            g: 0.08,
            b: 0.12,
            a: 1.0,
        }, // Very dark blue-gray
        menu: SerializableColor {
            r: 0.12,
            g: 0.12,
            b: 0.18,
            a: 1.0,
        }, // Slightly lighter for navigation
        sub_menu: SerializableColor {
            r: 0.16,
            g: 0.16,
            b: 0.22,
            a: 1.0,
        }, // Even lighter for sub-menus
        text: SerializableColor {
            r: 0.92,
            g: 0.92,
            b: 0.96,
            a: 1.0,
        }, // Light gray text
        text_secondary: SerializableColor {
            r: 0.7,
            g: 0.7,
            b: 0.8,
            a: 1.0,
        }, // Muted text
        primary: SerializableColor {
            r: 0.2,
            g: 0.6,
            b: 1.0,
            a: 1.0,
        }, // Network blue
        success: SerializableColor {
            r: 0.0,
            g: 0.8,
            b: 0.4,
            a: 1.0,
        }, // Green for success
        warning: SerializableColor {
            r: 1.0,
            g: 0.7,
            b: 0.0,
            a: 1.0,
        }, // Orange for warnings
        danger: SerializableColor {
            r: 1.0,
            g: 0.3,
            b: 0.3,
            a: 1.0,
        }, // Red for errors
        border: SerializableColor {
            r: 0.3,
            g: 0.3,
            b: 0.4,
            a: 1.0,
        }, // Subtle borders
        border_focused: SerializableColor {
            r: 0.2,
            g: 0.6,
            b: 1.0,
            a: 1.0,
        }, // Focused border (blue)
        border_hover: SerializableColor {
            r: 0.4,
            g: 0.4,
            b: 0.5,
            a: 1.0,
        }, // Hover border
        border_disabled: SerializableColor {
            r: 0.2,
            g: 0.2,
            b: 0.25,
            a: 1.0,
        }, // Disabled border
        active: SerializableColor {
            r: 0.3,
            g: 0.7,
            b: 1.0,
            a: 1.0,
        }, // Active selection
        hover: SerializableColor {
            r: 0.2,
            g: 0.2,
            b: 0.28,
            a: 1.0,
        }, // Hover states
    };

    /// Light theme color scheme - alternative for bright environments
    pub const LIGHT: Self = Self {
        background: SerializableColor {
            r: 0.98,
            g: 0.98,
            b: 1.0,
            a: 1.0,
        }, // Very light blue-white
        menu: SerializableColor {
            r: 0.94,
            g: 0.94,
            b: 0.98,
            a: 1.0,
        }, // Light gray menu
        sub_menu: SerializableColor {
            r: 0.9,
            g: 0.9,
            b: 0.96,
            a: 1.0,
        }, // Slightly darker sub-menu
        text: SerializableColor {
            r: 0.1,
            g: 0.1,
            b: 0.15,
            a: 1.0,
        }, // Dark text
        text_secondary: SerializableColor {
            r: 0.4,
            g: 0.4,
            b: 0.5,
            a: 1.0,
        }, // Muted dark text
        primary: SerializableColor {
            r: 0.1,
            g: 0.4,
            b: 0.8,
            a: 1.0,
        }, // Darker blue for contrast
        success: SerializableColor {
            r: 0.0,
            g: 0.6,
            b: 0.3,
            a: 1.0,
        }, // Darker green
        warning: SerializableColor {
            r: 0.8,
            g: 0.5,
            b: 0.0,
            a: 1.0,
        }, // Darker orange
        danger: SerializableColor {
            r: 0.8,
            g: 0.2,
            b: 0.2,
            a: 1.0,
        }, // Darker red
        border: SerializableColor {
            r: 0.7,
            g: 0.7,
            b: 0.8,
            a: 1.0,
        }, // Visible borders
        border_focused: SerializableColor {
            r: 0.1,
            g: 0.4,
            b: 0.8,
            a: 1.0,
        }, // Focused border (darker blue)
        border_hover: SerializableColor {
            r: 0.6,
            g: 0.6,
            b: 0.7,
            a: 1.0,
        }, // Hover border
        border_disabled: SerializableColor {
            r: 0.8,
            g: 0.8,
            b: 0.85,
            a: 1.0,
        }, // Disabled border
        active: SerializableColor {
            r: 0.2,
            g: 0.5,
            b: 0.9,
            a: 1.0,
        }, // Active selection
        hover: SerializableColor {
            r: 0.88,
            g: 0.88,
            b: 0.94,
            a: 1.0,
        }, // Light hover
    };

    /// High contrast theme for accessibility
    pub const HIGH_CONTRAST: Self = Self {
        background: SerializableColor {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        },
        menu: SerializableColor {
            r: 0.1,
            g: 0.1,
            b: 0.1,
            a: 1.0,
        },
        sub_menu: SerializableColor {
            r: 0.2,
            g: 0.2,
            b: 0.2,
            a: 1.0,
        },
        text: SerializableColor {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        },
        text_secondary: SerializableColor {
            r: 0.8,
            g: 0.8,
            b: 0.8,
            a: 1.0,
        },
        primary: SerializableColor {
            r: 0.0,
            g: 0.7,
            b: 1.0,
            a: 1.0,
        },
        success: SerializableColor {
            r: 0.0,
            g: 1.0,
            b: 0.5,
            a: 1.0,
        },
        warning: SerializableColor {
            r: 1.0,
            g: 1.0,
            b: 0.0,
            a: 1.0,
        },
        danger: SerializableColor {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        },
        border: SerializableColor {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        },
        border_focused: SerializableColor {
            r: 0.0,
            g: 0.9,
            b: 1.0,
            a: 1.0,
        }, // Bright focused border
        border_hover: SerializableColor {
            r: 0.8,
            g: 0.8,
            b: 0.8,
            a: 1.0,
        }, // Light hover border
        border_disabled: SerializableColor {
            r: 0.4,
            g: 0.4,
            b: 0.4,
            a: 1.0,
        }, // Gray disabled border
        active: SerializableColor {
            r: 0.0,
            g: 0.8,
            b: 1.0,
            a: 1.0,
        },
        hover: SerializableColor {
            r: 0.3,
            g: 0.3,
            b: 0.3,
            a: 1.0,
        },
    };
}

/// Theme definition loaded from JSON
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ThemeDefinition {
    pub name: String,
    pub description: String,
    pub colors: NetMonkeyColors,
    pub is_dark: bool,
}

/// Custom theme variants for the NetMonkey application
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum NetMonkeyTheme {
    Loaded(String), // Theme name/ID
}

impl Default for NetMonkeyTheme {
    fn default() -> Self {
        NetMonkeyTheme::Loaded("Dark".to_string())
    }
}

impl NetMonkeyTheme {
    /// Get the color palette for this theme
    pub fn colors(&self) -> NetMonkeyColors {
        match self {
            NetMonkeyTheme::Loaded(theme_name) => ThemeManager::load_theme(theme_name)
                .map(|def| def.colors)
                .unwrap_or(NetMonkeyColors::DARK),
        }
    }

    /// Convert to an iced Theme
    pub fn to_iced_theme(self) -> Theme {
        let colors = self.colors();

        let palette = Palette {
            background: colors.background.into(),
            text: colors.text.into(),
            primary: colors.primary.into(),
            success: colors.success.into(),
            danger: colors.danger.into(),
        };

        Theme::custom(self.name().to_string(), palette)
    }

    /// Create extended palette with detailed color control
    pub fn to_extended_iced_theme(self) -> Theme {
        let colors = self.colors();
        let theme_def = match &self {
            NetMonkeyTheme::Loaded(theme_name) => ThemeManager::load_theme(theme_name)
                .unwrap_or_else(|| ThemeDefinition {
                    name: "Dark".to_string(),
                    description: "Default dark theme".to_string(),
                    colors: NetMonkeyColors::DARK,
                    is_dark: true,
                }),
        };

        let palette = Palette {
            background: colors.background.into(),
            text: colors.text.into(),
            primary: colors.primary.into(),
            success: colors.success.into(),
            danger: colors.danger.into(),
        };

        Theme::custom_with_fn(self.name().to_string(), palette, move |_| Extended {
            background: iced::theme::palette::Background {
                base: iced::theme::palette::Pair {
                    color: colors.background.into(),
                    text: colors.text.into(),
                },
                weak: iced::theme::palette::Pair {
                    color: colors.menu.into(),
                    text: colors.text.into(),
                },
                strong: iced::theme::palette::Pair {
                    color: colors.active.into(),
                    text: colors.text.into(),
                },
            },
            primary: iced::theme::palette::Primary {
                base: iced::theme::palette::Pair {
                    color: colors.primary.into(),
                    text: colors.background.into(),
                },
                weak: iced::theme::palette::Pair {
                    color: colors.active.into(),
                    text: colors.text.into(),
                },
                strong: iced::theme::palette::Pair {
                    color: colors.primary.into(),
                    text: colors.background.into(),
                },
            },
            secondary: iced::theme::palette::Secondary {
                base: iced::theme::palette::Pair {
                    color: colors.sub_menu.into(),
                    text: colors.text.into(),
                },
                weak: iced::theme::palette::Pair {
                    color: colors.hover.into(),
                    text: colors.text_secondary.into(),
                },
                strong: iced::theme::palette::Pair {
                    color: colors.sub_menu.into(),
                    text: colors.text.into(),
                },
            },
            success: iced::theme::palette::Success {
                base: iced::theme::palette::Pair {
                    color: colors.success.into(),
                    text: colors.background.into(),
                },
                weak: iced::theme::palette::Pair {
                    color: Color::from_rgba(
                        colors.success.r,
                        colors.success.g,
                        colors.success.b,
                        0.3,
                    ),
                    text: colors.text.into(),
                },
                strong: iced::theme::palette::Pair {
                    color: colors.success.into(),
                    text: colors.background.into(),
                },
            },
            danger: iced::theme::palette::Danger {
                base: iced::theme::palette::Pair {
                    color: colors.danger.into(),
                    text: colors.background.into(),
                },
                weak: iced::theme::palette::Pair {
                    color: Color::from_rgba(colors.danger.r, colors.danger.g, colors.danger.b, 0.3),
                    text: colors.text.into(),
                },
                strong: iced::theme::palette::Pair {
                    color: colors.danger.into(),
                    text: colors.background.into(),
                },
            },
            is_dark: theme_def.is_dark,
        })
    }

    /// Get all available themes
    pub fn all() -> Vec<Self> {
        ThemeManager::available_themes()
            .into_iter()
            .map(NetMonkeyTheme::Loaded)
            .collect()
    }

    /// Get theme name as string
    pub fn name(&self) -> String {
        match self {
            NetMonkeyTheme::Loaded(theme_name) => ThemeManager::load_theme(theme_name)
                .map(|def| def.name)
                .unwrap_or_else(|| theme_name.clone()),
        }
    }

    /// Get theme description for better UX
    pub fn description(&self) -> String {
        match self {
            NetMonkeyTheme::Loaded(theme_name) => ThemeManager::load_theme(theme_name)
                .map(|def| def.description)
                .unwrap_or_else(|| "Unknown theme".to_string()),
        }
    }
}

impl From<NetMonkeyTheme> for Theme {
    fn from(theme: NetMonkeyTheme) -> Self {
        theme.to_extended_iced_theme()
    }
}

impl Display for NetMonkeyTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Theme manager for loading and managing themes from JSON files
pub struct ThemeManager;

impl ThemeManager {
    /// Get the themes directory path based on build mode
    fn themes_dir() -> String {
        #[cfg(debug_assertions)]
        {
            // In debug mode, find the workspace root and use app/data/themes
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
                        return "app/data/themes".to_string();
                    }
                }
                path.push("app");
                path.push("data");
                path.push("themes");
                path.to_string_lossy().to_string()
            } else {
                "app/data/themes".to_string()
            }
        }
        #[cfg(not(debug_assertions))]
        {
            // In release mode, use current working directory
            "data/themes".to_string()
        }
    }

    /// Load a theme from a JSON file
    pub fn load_theme(theme_name: &str) -> Option<ThemeDefinition> {
        let file_path = format!(
            "{}/{}.json",
            Self::themes_dir(),
            theme_name.replace(" ", "_").to_lowercase()
        );

        match fs::read_to_string(&file_path) {
            Ok(content) => match serde_json::from_str::<ThemeDefinition>(&content) {
                Ok(theme) => Some(theme),
                Err(e) => {
                    eprintln!("Error parsing theme {theme_name}: {e}");
                    None
                }
            },
            Err(e) => {
                eprintln!("Error reading theme file {file_path}: {e}");
                None
            }
        }
    }

    /// Save a theme to a JSON file
    pub fn save_theme(theme: &ThemeDefinition) -> Result<(), Box<dyn std::error::Error>> {
        let themes_dir = Self::themes_dir();
        // Ensure themes directory exists
        fs::create_dir_all(&themes_dir)?;

        let file_name = theme.name.replace(" ", "_").to_lowercase();
        let file_path = format!("{themes_dir}/{file_name}.json");

        let json = serde_json::to_string_pretty(theme)?;
        fs::write(&file_path, json)?;

        Ok(())
    }

    /// Get list of available theme names
    pub fn available_themes() -> Vec<String> {
        let mut themes = Vec::new();

        if let Ok(entries) = fs::read_dir(Self::themes_dir()) {
            for entry in entries.flatten() {
                if let Some(file_name) = entry.file_name().to_str()
                    && file_name.ends_with(".json")
                {
                    let theme_name = file_name.strip_suffix(".json").unwrap_or(file_name);
                    // Convert back from file name format to display name with proper capitalization
                    let display_name = theme_name
                        .replace("_", " ")
                        .split_whitespace()
                        .map(|word| {
                            let mut chars = word.chars();
                            match chars.next() {
                                None => String::new(),
                                Some(first) => {
                                    first.to_uppercase().collect::<String>() + chars.as_str()
                                }
                            }
                        })
                        .collect::<Vec<String>>()
                        .join(" ");
                    themes.push(display_name);
                }
            }
        }

        // If no themes found, return defaults
        if themes.is_empty() {
            themes = vec![
                "Dark".to_string(),
                "Light".to_string(),
                "High Contrast".to_string(),
            ];
        }

        themes.sort();
        themes
    }

    /// Create default themes if they don't exist
    pub fn ensure_default_themes() {
        let _ = Self::save_theme(&ThemeDefinition {
            name: "Dark".to_string(),
            description: "Dark blue theme optimized for network monitoring".to_string(),
            colors: NetMonkeyColors::DARK,
            is_dark: true,
        });

        let _ = Self::save_theme(&ThemeDefinition {
            name: "Light".to_string(),
            description: "Light theme for bright environments".to_string(),
            colors: NetMonkeyColors::LIGHT,
            is_dark: false,
        });

        let _ = Self::save_theme(&ThemeDefinition {
            name: "High Contrast".to_string(),
            description: "High contrast theme for accessibility".to_string(),
            colors: NetMonkeyColors::HIGH_CONTRAST,
            is_dark: true,
        });
    }

    /// Clean up temporary themes created during editing
    pub fn cleanup_temporary_themes() {
        if let Ok(entries) = fs::read_dir(Self::themes_dir()) {
            for entry in entries.flatten() {
                if let Some(file_name) = entry.file_name().to_str()
                    && file_name.ends_with(".json")
                {
                    let theme_name = file_name.strip_suffix(".json").unwrap_or(file_name);
                    // Remove temporary editing themes
                    if theme_name.starts_with("editing_") {
                        let _ = fs::remove_file(entry.path());
                    }
                }
            }
        }
    }

    /// Debug function to print the current themes directory path
    pub fn debug_paths() {
        let themes_dir = Self::themes_dir();
        println!("Themes directory: {themes_dir}");
        println!(
            "Themes directory exists: {}",
            std::path::Path::new(&themes_dir).exists()
        );

        if let Ok(entries) = fs::read_dir(&themes_dir) {
            println!("Available theme files:");
            for entry in entries.flatten() {
                if let Some(file_name) = entry.file_name().to_str()
                    && file_name.ends_with(".json")
                {
                    println!("  - {file_name}");
                }
            }
        } else {
            println!("Could not read themes directory");
        }
    }
}

/// Helper functions for creating themed components
pub mod helpers {
    use super::*;
    use iced::widget::{container, text};

    /// Create a menu container with appropriate background
    #[allow(dead_code)]
    pub fn menu_container<'a, Message>(
        content: impl Into<iced::Element<'a, Message>>,
        theme: NetMonkeyTheme,
    ) -> container::Container<'a, Message> {
        let colors = theme.colors();
        container(content)
            .style(move |_| container::Style {
                background: Some(iced::Background::Color(colors.background.into())),
                border: iced::Border {
                    color: colors.border.into(),
                    width: 0.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            })
            .padding(8)
    }

    /// Create a sub-menu container
    #[allow(dead_code)]
    pub fn sub_menu_container<'a, Message>(
        content: impl Into<iced::Element<'a, Message>>,
        theme: NetMonkeyTheme,
    ) -> container::Container<'a, Message> {
        let colors = theme.colors();
        container(content)
            .style(move |_| container::Style {
                background: Some(iced::Background::Color(colors.sub_menu.into())),
                border: iced::Border {
                    color: colors.border_focused.into(),
                    width: 1.5,
                    radius: 6.0.into(),
                },
                ..Default::default()
            })
            .padding(6)
    }

    /// Create themed status text (success, warning, danger)
    #[allow(dead_code)]
    pub fn status_text<'a>(
        content: impl ToString,
        status: StatusType,
        theme: NetMonkeyTheme,
    ) -> text::Text<'a> {
        let colors = theme.colors();
        let color: Color = match status {
            StatusType::Success => colors.success.into(),
            StatusType::Warning => colors.warning.into(),
            StatusType::Danger => colors.danger.into(),
            StatusType::Info => colors.primary.into(),
        };

        text(content.to_string()).color(color)
    }

    /// Create a themed container with focus-aware borders
    #[allow(dead_code)]
    pub fn themed_container<'a, Message>(
        content: impl Into<iced::Element<'a, Message>>,
        theme: NetMonkeyTheme,
        is_focused: bool,
        is_hovered: bool,
    ) -> container::Container<'a, Message> {
        let colors = theme.colors();
        let border_color = if is_focused {
            colors.border_focused
        } else if is_hovered {
            colors.border_hover
        } else {
            colors.border
        };

        container(content)
            .style(move |_| container::Style {
                background: Some(iced::Background::Color(colors.background.into())),
                border: iced::Border {
                    color: border_color.into(),
                    width: if is_focused { 2.0 } else { 1.0 },
                    radius: 4.0.into(),
                },
                ..Default::default()
            })
            .padding(8)
    }

    /// Create a themed input container with state-aware styling
    #[allow(dead_code)]
    pub fn themed_input_container<'a, Message>(
        content: impl Into<iced::Element<'a, Message>>,
        theme: NetMonkeyTheme,
        is_focused: bool,
        is_disabled: bool,
    ) -> container::Container<'a, Message> {
        let colors = theme.colors();
        let (border_color, border_width, background_color) = if is_disabled {
            (
                colors.border_disabled,
                1.0,
                Color {
                    r: colors.background.r * 0.9,
                    g: colors.background.g * 0.9,
                    b: colors.background.b * 0.9,
                    a: colors.background.a,
                },
            )
        } else if is_focused {
            (colors.border_focused, 2.0, colors.background.into())
        } else {
            (colors.border, 1.0, colors.background.into())
        };

        container(content)
            .style(move |_| container::Style {
                background: Some(iced::Background::Color(background_color)),
                border: iced::Border {
                    color: border_color.into(),
                    width: border_width,
                    radius: 4.0.into(),
                },
                ..Default::default()
            })
            .padding(8)
    }

    #[derive(Debug, Clone, Copy)]
    #[allow(dead_code)]
    pub enum StatusType {
        Success,
        Warning,
        Danger,
        Info,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_creation() {
        let dark_theme = NetMonkeyTheme::Loaded("Dark".to_string());
        let iced_theme = dark_theme.to_iced_theme();

        // Verify theme conversion works
        assert!(matches!(iced_theme, Theme::Custom(_)));
    }

    #[test]
    fn test_color_accessibility() {
        let colors = NetMonkeyColors::DARK;

        // Basic contrast checks (simplified)
        assert!(colors.text.r > 0.5); // Light text
        assert!(colors.background.r < 0.2); // Dark background
    }

    #[test]
    fn test_theme_manager() {
        // Ensure default themes exist
        ThemeManager::ensure_default_themes();

        // Test loading a theme
        let dark_theme = ThemeManager::load_theme("Dark");
        assert!(dark_theme.is_some());

        if let Some(theme) = dark_theme {
            assert_eq!(theme.name, "Dark");
            assert!(theme.is_dark);
        }

        // Test available themes
        let available = ThemeManager::available_themes();
        assert!(!available.is_empty());
    }

    #[test]
    fn test_all_themes() {
        ThemeManager::ensure_default_themes();
        let themes = NetMonkeyTheme::all();
        assert!(!themes.is_empty());

        for theme in themes {
            let _ = theme.to_iced_theme(); // Should not panic
        }
    }

    #[test]
    fn test_path_resolution() {
        let themes_dir = ThemeManager::themes_dir();

        #[cfg(debug_assertions)]
        {
            // In debug mode, should point to app/data/themes
            assert!(themes_dir.contains("app"));
            assert!(themes_dir.ends_with("data/themes") || themes_dir.ends_with("data\\themes"));
        }

        #[cfg(not(debug_assertions))]
        {
            // In release mode, should be relative to cwd
            assert_eq!(themes_dir, "data/themes");
        }

        println!("Resolved themes directory: {themes_dir}");
    }
}
