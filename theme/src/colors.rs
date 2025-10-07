//! Custom theme module for the NetMonkey network monitoring application.
//!
//! This module provides custom color palettes and themes optimized for
//! network monitoring interfaces, with distinct colors for different
//! UI sections like backgrounds, menus, and sub-menus.

use std::fmt::Display;

use iced::theme::{Palette, palette::Extended};
use iced::{Color, Theme};

/// Custom color palette for NetMonkey application
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NetMonkeyColors {
    /// Main application background
    pub background: Color,
    /// Menu/navigation background
    pub menu: Color,
    /// Sub-menu background
    pub sub_menu: Color,
    /// Primary text color
    pub text: Color,
    /// Secondary/muted text color
    pub text_secondary: Color,
    /// Primary accent color (for buttons, highlights)
    pub primary: Color,
    /// Success color (for successful connections, scans)
    pub success: Color,
    /// Warning color (for timeouts, warnings)
    pub warning: Color,
    /// Error/danger color (for failed connections, errors)
    pub danger: Color,
    /// Border color for components
    pub border: Color,
    /// Active/selected state color
    pub active: Color,
    /// Hover state color
    pub hover: Color,
}

impl NetMonkeyColors {
    /// Dark theme color scheme - recommended for network monitoring
    pub const DARK: Self = Self {
        background: Color::from_rgb(0.08, 0.08, 0.12), // Very dark blue-gray
        menu: Color::from_rgb(0.12, 0.12, 0.18),       // Slightly lighter for navigation
        sub_menu: Color::from_rgb(0.16, 0.16, 0.22),   // Even lighter for sub-menus
        text: Color::from_rgb(0.92, 0.92, 0.96),       // Light gray text
        text_secondary: Color::from_rgb(0.7, 0.7, 0.8), // Muted text
        primary: Color::from_rgb(0.2, 0.6, 1.0),       // Network blue
        success: Color::from_rgb(0.0, 0.8, 0.4),       // Green for success
        warning: Color::from_rgb(1.0, 0.7, 0.0),       // Orange for warnings
        danger: Color::from_rgb(1.0, 0.3, 0.3),        // Red for errors
        border: Color::from_rgb(0.3, 0.3, 0.4),        // Subtle borders
        active: Color::from_rgb(0.3, 0.7, 1.0),        // Active selection
        hover: Color::from_rgb(0.2, 0.2, 0.28),        // Hover states
    };

    /// Light theme color scheme - alternative for bright environments
    pub const LIGHT: Self = Self {
        background: Color::from_rgb(0.98, 0.98, 1.0), // Very light blue-white
        menu: Color::from_rgb(0.94, 0.94, 0.98),      // Light gray menu
        sub_menu: Color::from_rgb(0.9, 0.9, 0.96),    // Slightly darker sub-menu
        text: Color::from_rgb(0.1, 0.1, 0.15),        // Dark text
        text_secondary: Color::from_rgb(0.4, 0.4, 0.5), // Muted dark text
        primary: Color::from_rgb(0.1, 0.4, 0.8),      // Darker blue for contrast
        success: Color::from_rgb(0.0, 0.6, 0.3),      // Darker green
        warning: Color::from_rgb(0.8, 0.5, 0.0),      // Darker orange
        danger: Color::from_rgb(0.8, 0.2, 0.2),       // Darker red
        border: Color::from_rgb(0.7, 0.7, 0.8),       // Visible borders
        active: Color::from_rgb(0.2, 0.5, 0.9),       // Active selection
        hover: Color::from_rgb(0.88, 0.88, 0.94),     // Light hover
    };

    /// High contrast theme for accessibility
    pub const HIGH_CONTRAST: Self = Self {
        background: Color::BLACK,
        menu: Color::from_rgb(0.1, 0.1, 0.1),
        sub_menu: Color::from_rgb(0.2, 0.2, 0.2),
        text: Color::WHITE,
        text_secondary: Color::from_rgb(0.8, 0.8, 0.8),
        primary: Color::from_rgb(0.0, 0.7, 1.0),
        success: Color::from_rgb(0.0, 1.0, 0.5),
        warning: Color::from_rgb(1.0, 1.0, 0.0),
        danger: Color::from_rgb(1.0, 0.0, 0.0),
        border: Color::WHITE,
        active: Color::from_rgb(0.0, 0.8, 1.0),
        hover: Color::from_rgb(0.3, 0.3, 0.3),
    };
}

/// Custom theme variants for the NetMonkey application
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
pub enum NetMonkeyTheme {
    #[default]
    Dark,
    Light,
    HighContrast,
}

impl NetMonkeyTheme {
    /// Get the color palette for this theme
    pub fn colors(&self) -> NetMonkeyColors {
        match self {
            NetMonkeyTheme::Dark => NetMonkeyColors::DARK,
            NetMonkeyTheme::Light => NetMonkeyColors::LIGHT,
            NetMonkeyTheme::HighContrast => NetMonkeyColors::HIGH_CONTRAST,
        }
    }

    /// Convert to an iced Theme
    pub fn to_iced_theme(self) -> Theme {
        let colors = self.colors();

        let palette = Palette {
            background: colors.background,
            text: colors.text,
            primary: colors.primary,
            success: colors.success,
            danger: colors.danger,
        };

        Theme::custom(self.name().to_string(), palette)
    }

    /// Create extended palette with detailed color control
    pub fn to_extended_iced_theme(self) -> Theme {
        let colors = self.colors();

        let palette = Palette {
            background: colors.background,
            text: colors.text,
            primary: colors.primary,
            success: colors.success,
            danger: colors.danger,
        };

        Theme::custom_with_fn(self.name().to_string(), palette, move |_| Extended {
            background: iced::theme::palette::Background {
                base: iced::theme::palette::Pair {
                    color: colors.background,
                    text: colors.text,
                },
                weak: iced::theme::palette::Pair {
                    color: colors.menu,
                    text: colors.text,
                },
                strong: iced::theme::palette::Pair {
                    color: colors.active,
                    text: colors.text,
                },
            },
            primary: iced::theme::palette::Primary {
                base: iced::theme::palette::Pair {
                    color: colors.primary,
                    text: colors.background,
                },
                weak: iced::theme::palette::Pair {
                    color: colors.active,
                    text: colors.text,
                },
                strong: iced::theme::palette::Pair {
                    color: colors.primary,
                    text: colors.background,
                },
            },
            secondary: iced::theme::palette::Secondary {
                base: iced::theme::palette::Pair {
                    color: colors.sub_menu,
                    text: colors.text,
                },
                weak: iced::theme::palette::Pair {
                    color: colors.hover,
                    text: colors.text_secondary,
                },
                strong: iced::theme::palette::Pair {
                    color: colors.sub_menu,
                    text: colors.text,
                },
            },
            success: iced::theme::palette::Success {
                base: iced::theme::palette::Pair {
                    color: colors.success,
                    text: colors.background,
                },
                weak: iced::theme::palette::Pair {
                    color: Color::from_rgba(
                        colors.success.r,
                        colors.success.g,
                        colors.success.b,
                        0.3,
                    ),
                    text: colors.text,
                },
                strong: iced::theme::palette::Pair {
                    color: colors.success,
                    text: colors.background,
                },
            },
            danger: iced::theme::palette::Danger {
                base: iced::theme::palette::Pair {
                    color: colors.danger,
                    text: colors.background,
                },
                weak: iced::theme::palette::Pair {
                    color: Color::from_rgba(colors.danger.r, colors.danger.g, colors.danger.b, 0.3),
                    text: colors.text,
                },
                strong: iced::theme::palette::Pair {
                    color: colors.danger,
                    text: colors.background,
                },
            },
            is_dark: matches!(self, NetMonkeyTheme::Dark | NetMonkeyTheme::HighContrast),
        })
    }

    /// Get all available themes
    pub fn all() -> Vec<Self> {
        vec![
            NetMonkeyTheme::Dark,
            NetMonkeyTheme::Light,
            NetMonkeyTheme::HighContrast,
        ]
    }

    /// Get theme name as string
    pub fn name(&self) -> &'static str {
        match self {
            NetMonkeyTheme::Dark => "Dark",
            NetMonkeyTheme::Light => "Light",
            NetMonkeyTheme::HighContrast => "High Contrast",
        }
    }

    /// Get theme description for better UX
    pub fn description(&self) -> &'static str {
        match self {
            NetMonkeyTheme::Dark => "Dark blue theme optimized for network monitoring",
            NetMonkeyTheme::Light => "Light theme for bright environments",
            NetMonkeyTheme::HighContrast => "High contrast theme for accessibility",
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
        match self {
            NetMonkeyTheme::Dark => write!(f, "Dark"),
            NetMonkeyTheme::Light => write!(f, "Light"),
            NetMonkeyTheme::HighContrast => write!(f, "High Contrast"),
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
                background: Some(iced::Background::Color(colors.menu)),
                border: iced::Border {
                    color: colors.border,
                    width: 1.0,
                    radius: 4.0.into(),
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
                background: Some(iced::Background::Color(colors.sub_menu)),
                border: iced::Border {
                    color: colors.border,
                    width: 1.0,
                    radius: 2.0.into(),
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
        let color = match status {
            StatusType::Success => colors.success,
            StatusType::Warning => colors.warning,
            StatusType::Danger => colors.danger,
            StatusType::Info => colors.primary,
        };

        text(content.to_string()).color(color)
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
        let dark_theme = NetMonkeyTheme::Dark;
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
    fn test_all_themes() {
        let themes = NetMonkeyTheme::all();
        assert_eq!(themes.len(), 3);

        for theme in themes {
            let _ = theme.to_iced_theme(); // Should not panic
        }
    }
}
