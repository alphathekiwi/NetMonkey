//! Net Monkey Theme Library - COSMIC Integration
//!
//! This crate provides simple theming functionality that integrates directly
//! with the COSMIC desktop environment when available, and provides minimal
//! fallback themes for non-COSMIC environments.

#[cfg(feature = "cosmic")]
use libcosmic;
#[cfg(feature = "cosmic")]
use palette;

use iced::Theme;
use serde::{Deserialize, Serialize};

/// Simple color structure for basic theming needs
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SimpleColors {
    pub background: [f32; 4], // RGBA
    pub text: [f32; 4],
    pub primary: [f32; 4],
    pub success: [f32; 4],
    pub warning: [f32; 4],
    pub danger: [f32; 4],
}

impl SimpleColors {
    /// Convert to iced::Color
    pub fn background_color(&self) -> iced::Color {
        iced::Color::from_rgba(
            self.background[0],
            self.background[1],
            self.background[2],
            self.background[3],
        )
    }

    pub fn text_color(&self) -> iced::Color {
        iced::Color::from_rgba(self.text[0], self.text[1], self.text[2], self.text[3])
    }

    pub fn primary_color(&self) -> iced::Color {
        iced::Color::from_rgba(
            self.primary[0],
            self.primary[1],
            self.primary[2],
            self.primary[3],
        )
    }

    pub fn success_color(&self) -> iced::Color {
        iced::Color::from_rgba(
            self.success[0],
            self.success[1],
            self.success[2],
            self.success[3],
        )
    }

    pub fn warning_color(&self) -> iced::Color {
        iced::Color::from_rgba(
            self.warning[0],
            self.warning[1],
            self.warning[2],
            self.warning[3],
        )
    }

    pub fn danger_color(&self) -> iced::Color {
        iced::Color::from_rgba(
            self.danger[0],
            self.danger[1],
            self.danger[2],
            self.danger[3],
        )
    }
    /// Get a lighter version of primary color for hover/secondary elements
    pub fn primary_light(&self) -> iced::Color {
        let primary = self.primary_color();
        iced::Color::from_rgba(
            (primary.r * 1.2).min(1.0),
            (primary.g * 1.2).min(1.0),
            (primary.b * 1.2).min(1.0),
            primary.a * 0.2,
        )
    }

    /// Get menu/container background color (slightly different from main background)
    pub fn container_color(&self) -> iced::Color {
        let bg = self.background_color();
        if self.is_dark() {
            // Lighter for dark themes
            iced::Color::from_rgba(
                (bg.r + 0.05).min(1.0),
                (bg.g + 0.05).min(1.0),
                (bg.b + 0.05).min(1.0),
                bg.a,
            )
        } else {
            // Darker for light themes
            iced::Color::from_rgba(
                (bg.r - 0.05).max(0.0),
                (bg.g - 0.05).max(0.0),
                (bg.b - 0.05).max(0.0),
                bg.a,
            )
        }
    }

    /// Get border color
    pub fn border_color(&self) -> iced::Color {
        let text = self.text_color();
        iced::Color::from_rgba(text.r, text.g, text.b, 0.3)
    }

    /// Check if this is a dark theme
    pub fn is_dark(&self) -> bool {
        self.background[0] + self.background[1] + self.background[2] < 1.5
    }

    /// Dark theme colors (fallback)
    pub const DARK: Self = Self {
        background: [0.1, 0.1, 0.1, 1.0],
        text: [0.9, 0.9, 0.9, 1.0],
        primary: [0.2, 0.6, 1.0, 1.0],
        success: [0.2, 0.8, 0.2, 1.0],
        warning: [1.0, 0.6, 0.0, 1.0],
        danger: [1.0, 0.2, 0.2, 1.0],
    };

    /// Light theme colors (fallback)
    pub const LIGHT: Self = Self {
        background: [0.95, 0.95, 0.95, 1.0],
        text: [0.1, 0.1, 0.1, 1.0],
        primary: [0.0, 0.4, 0.8, 1.0],
        success: [0.0, 0.6, 0.0, 1.0],
        warning: [0.8, 0.4, 0.0, 1.0],
        danger: [0.8, 0.0, 0.0, 1.0],
    };
}

/// Theme provider that uses COSMIC when available, falls back to simple themes
#[derive(Debug, Clone)]
pub enum ThemeProvider {
    #[cfg(feature = "cosmic")]
    Cosmic(libcosmic::theme::Theme),
    Fallback(SimpleColors),
}

impl Default for ThemeProvider {
    fn default() -> Self {
        #[cfg(feature = "cosmic")]
        {
            // Try to get COSMIC theme if available
            if is_cosmic_environment() {
                if let Ok(cosmic_theme) = libcosmic::theme::Theme::get_active() {
                    return Self::Cosmic(cosmic_theme);
                }
            }
        }

        // Fallback to dark theme
        Self::Fallback(SimpleColors::DARK)
    }
}

impl ThemeProvider {
    /// Create a new theme provider
    pub fn new() -> Self {
        Self::default()
    }

    /// Force use of fallback theme
    pub fn fallback(colors: SimpleColors) -> Self {
        Self::Fallback(colors)
    }

    /// Get colors for current theme
    pub fn colors(&self) -> SimpleColors {
        match self {
            #[cfg(feature = "cosmic")]
            Self::Cosmic(theme) => {
                // Convert COSMIC theme to simple colors
                SimpleColors {
                    background: cosmic_color_to_array(theme.bg_color()),
                    text: cosmic_color_to_array(theme.on_bg_color()),
                    primary: cosmic_color_to_array(theme.accent_color()),
                    success: cosmic_color_to_array(theme.success_color()),
                    warning: cosmic_color_to_array(theme.warning_color()),
                    danger: cosmic_color_to_array(theme.destructive_color()),
                }
            }
            Self::Fallback(colors) => *colors,
        }
    }

    /// Convert to iced Theme
    pub fn to_iced_theme(&self) -> Theme {
        let colors = self.colors();

        let palette = iced::theme::Palette {
            background: colors.background_color(),
            text: colors.text_color(),
            primary: colors.primary_color(),
            success: colors.success_color(),
            danger: colors.danger_color(),
        };

        Theme::custom("net_monkey".to_string(), palette)
    }

    /// Check if current theme is dark
    pub fn is_dark(&self) -> bool {
        match self {
            #[cfg(feature = "cosmic")]
            Self::Cosmic(theme) => theme.is_dark,
            Self::Fallback(colors) => {
                // Simple heuristic: if background is darker than middle gray
                colors.background[0] + colors.background[1] + colors.background[2] < 1.5
            }
        }
    }

    /// Get theme name
    pub fn name(&self) -> &str {
        match self {
            #[cfg(feature = "cosmic")]
            Self::Cosmic(theme) => &theme.name,
            Self::Fallback(_) => {
                if self.is_dark() {
                    "Dark"
                } else {
                    "Light"
                }
            }
        }
    }

    /// Refresh theme (useful for COSMIC theme changes)
    pub fn refresh(&mut self) -> Result<(), &'static str> {
        #[cfg(feature = "cosmic")]
        {
            if let Self::Cosmic(_) = self {
                match libcosmic::theme::Theme::get_active() {
                    Ok(new_theme) => {
                        *self = Self::Cosmic(new_theme);
                        return Ok(());
                    }
                    Err(_) => {
                        return Err("Failed to refresh COSMIC theme");
                    }
                }
            }
        }

        // Fallback themes don't need refreshing
        Ok(())
    }

    /// Check if COSMIC integration is active
    pub fn is_cosmic_active(&self) -> bool {
        #[cfg(feature = "cosmic")]
        {
            matches!(self, Self::Cosmic(_))
        }
        #[cfg(not(feature = "cosmic"))]
        {
            false
        }
    }
}

/// Helper function to check if we're in a COSMIC environment
pub fn is_cosmic_environment() -> bool {
    #[cfg(feature = "cosmic")]
    {
        std::env::var("XDG_CURRENT_DESKTOP")
            .map(|desktop| desktop.contains("COSMIC"))
            .unwrap_or(false)
            || std::env::var("XDG_SESSION_DESKTOP")
                .map(|session| session.contains("cosmic"))
                .unwrap_or(false)
            || std::env::var("COSMIC_SESSION").is_ok()
    }
    #[cfg(not(feature = "cosmic"))]
    {
        false
    }
}

/// Convert COSMIC color to array
#[cfg(feature = "cosmic")]
fn cosmic_color_to_array(color: palette::Srgba) -> [f32; 4] {
    [color.red, color.green, color.blue, color.alpha]
}

/// Helper functions for common theming operations
pub mod helpers {
    use super::*;
    use iced::Element;
    use iced::widget::{container, text};

    /// Create a themed container
    pub fn themed_container<'a, Message>(
        content: impl Into<Element<'a, Message>>,
        theme_provider: &ThemeProvider,
    ) -> container::Container<'a, Message>
    where
        Message: 'a,
    {
        let colors = theme_provider.colors();

        container(content).style(move |_theme| container::Style {
            background: Some(iced::Background::Color(colors.background_color())),
            text_color: Some(colors.text_color()),
            ..Default::default()
        })
    }

    /// Create themed text
    pub fn themed_text<'a, T>(content: T, theme_provider: &ThemeProvider) -> text::Text<'a>
    where
        T: iced::widget::text::IntoFragment<'a>,
    {
        let colors = theme_provider.colors();
        text(content).color(colors.text_color())
    }

    /// Status text with appropriate colors
    pub fn status_text<'a, T>(
        content: T,
        status: StatusType,
        theme_provider: &ThemeProvider,
    ) -> text::Text<'a>
    where
        T: iced::widget::text::IntoFragment<'a>,
    {
        let colors = theme_provider.colors();
        let color = match status {
            StatusType::Success => colors.success_color(),
            StatusType::Warning => colors.warning_color(),
            StatusType::Danger => colors.danger_color(),
            StatusType::Info => colors.primary_color(),
        };
        text(content).color(color)
    }

    /// Status types for themed components
    #[derive(Debug, Clone, Copy)]
    pub enum StatusType {
        Success,
        Warning,
        Danger,
        Info,
    }

    /// Legacy helper for menu containers (maps to themed_container)
    pub fn menu_container<'a, Message>(
        content: impl Into<Element<'a, Message>>,
        theme_provider: &ThemeProvider,
    ) -> container::Container<'a, Message>
    where
        Message: 'a,
    {
        themed_container(content, theme_provider)
    }

    /// Legacy helper for sub-menu containers (maps to themed_container)
    pub fn sub_menu_container<'a, Message>(
        content: impl Into<Element<'a, Message>>,
        theme_provider: &ThemeProvider,
    ) -> container::Container<'a, Message>
    where
        Message: 'a,
    {
        themed_container(content, theme_provider)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_provider_creation() {
        let provider = ThemeProvider::new();
        assert!(!provider.name().is_empty());
    }

    #[test]
    fn test_fallback_theme() {
        let provider = ThemeProvider::fallback(SimpleColors::DARK);
        let colors = provider.colors();
        assert_eq!(colors.background, SimpleColors::DARK.background);
    }

    #[test]
    fn test_iced_theme_conversion() {
        let provider = ThemeProvider::fallback(SimpleColors::LIGHT);
        let _iced_theme = provider.to_iced_theme();
        // If we get here without panic, conversion works
    }

    #[test]
    fn test_dark_detection() {
        let dark_provider = ThemeProvider::fallback(SimpleColors::DARK);
        let light_provider = ThemeProvider::fallback(SimpleColors::LIGHT);

        assert!(dark_provider.is_dark());
        assert!(!light_provider.is_dark());
    }

    #[cfg(feature = "cosmic")]
    #[test]
    fn test_cosmic_environment_detection() {
        // This function should not panic even if COSMIC is not available
        let _is_cosmic = is_cosmic_environment();
    }
}
