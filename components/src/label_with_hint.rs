use iced::widget::{container, row, text, tooltip};
use iced::{Color, Element, Length, Padding, Renderer, Theme};
use net_monkey_theme::NetMonkeyTheme;

/// A text label component with an optional help hint icon that shows a tooltip on hover.
///
/// This component displays a text label with a circular help icon that shows
/// hint text when hovered over. The help icon is positioned on the far right
/// of the component space. The tooltip appears below the entire text line when
/// hovering over the help icon.
///
/// # Features
/// - Text label with configurable size and styling
/// - Custom help hint icon with circle and "?" symbol that inherits text size
/// - Help icon positioned on the far right of the component space
/// - Tooltip display below the text when hovering over the help icon
/// - Full NetMonkey theme integration with automatic color adaptation
/// - Support for Dark, Light, and High Contrast themes
/// - Chainable builder pattern for configuration
/// - Professional monitoring application aesthetic
///
/// # Examples
///
/// ## Basic Usage
/// ```rust
/// let label = LabelWithHint::new(
///     "Ports List",
///     "Comma-separated list of ports to scan (e.g., 80, 443, 22)"
/// );
/// ```
///
/// ## Customized Usage with Theming
/// ```rust
/// let label = LabelWithHint::new(
///     "Network Interface",
///     "Select the network interface to monitor.\nUse 'auto' for automatic detection."
/// )
/// .text_size(20.0)  // Both label and help icon will use this size
/// .width(Length::Fixed(300.0))
/// .theme(NetMonkeyTheme::Dark);  // Apply custom theme
/// ```
pub struct LabelWithHint {
    label_text: String,
    hint_text: String,
    width: Length,
    text_size: f32,
    padding: Padding,
    text_color: Option<Color>,
    theme: NetMonkeyTheme,
}

impl LabelWithHint {
    /// Creates a new LabelWithHint component
    ///
    /// # Arguments
    /// * `label_text` - The main label text to display
    /// * `hint_text` - Help text to show in tooltip when hovering over the help icon
    pub fn new(label_text: impl Into<String>, hint_text: impl Into<String>) -> Self {
        Self {
            label_text: label_text.into(),
            hint_text: hint_text.into(),
            width: Length::Shrink,
            text_size: 14.0,
            padding: Padding::new(0.0),
            text_color: None,
            theme: NetMonkeyTheme::Dark,
        }
    }

    /// Sets the width of the component
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the text size for both the label and help icon
    pub fn text_size(mut self, size: f32) -> Self {
        self.text_size = size;
        self
    }

    /// Sets the padding around the component
    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    /// Sets the text color for the label
    pub fn color(mut self, color: Color) -> Self {
        self.text_color = Some(color);
        self
    }

    /// Sets the NetMonkey theme for the component
    ///
    /// This applies the appropriate color scheme including:
    /// - Help icon background and border colors
    /// - Tooltip background and text colors
    /// - Text colors that adapt to the theme
    ///
    /// # Arguments
    /// * `theme` - The NetMonkey theme variant to apply
    pub fn theme(mut self, theme: NetMonkeyTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Converts the component into an Element
    pub fn into_element<Message>(self) -> Element<'static, Message, Theme, Renderer>
    where
        Message: Clone + 'static,
    {
        let colors = self.theme.colors();

        let label = if let Some(color) = self.text_color {
            text(self.label_text.clone())
                .size(self.text_size)
                .color(color)
        } else {
            text(self.label_text.clone())
                .size(self.text_size)
                .color(colors.text)
        };

        if !self.hint_text.is_empty() {
            let text_size = self.text_size;
            let hint_text = self.hint_text.clone();

            // Create a simple help icon using container with NetMonkey theming
            let help_icon = container(text("?").size(text_size * 0.8).color(Color::WHITE))
                .width(Length::Fixed(text_size))
                .height(Length::Fixed(text_size))
                .padding(Padding::new(text_size * 0.1))
                .style(move |_theme: &Theme| container::Style {
                    background: Some(iced::Background::Color(colors.primary)),
                    border: iced::Border {
                        color: Color::from_rgb(
                            colors.primary.r * 0.75,
                            colors.primary.g * 0.75,
                            colors.primary.b * 0.75,
                        ),
                        width: 1.0,
                        radius: (text_size / 2.0).into(),
                    },
                    text_color: Some(Color::WHITE),
                    shadow: iced::Shadow::default(),
                });

            // Wrap help icon with tooltip using NetMonkey theming
            let help_icon_with_tooltip = tooltip(
                help_icon,
                container(text(hint_text).size(12.0).color(colors.text))
                    .padding(8.0)
                    .style(move |_theme: &Theme| container::Style {
                        text_color: Some(colors.text),
                        background: Some(iced::Background::Color(colors.menu)),
                        border: iced::Border {
                            color: colors.primary,
                            width: 1.5,
                            radius: 6.0.into(),
                        },
                        shadow: iced::Shadow {
                            color: Color::from_rgba(0.0, 0.0, 0.0, 0.4),
                            offset: iced::Vector::new(0.0, 3.0),
                            blur_radius: 8.0,
                        },
                    }),
                tooltip::Position::Left,
            );

            // Use Fill width for label and Shrink for icon to push icon to the right
            let content = row![
                container(label)
                    .width(Length::Fill)
                    .align_x(iced::alignment::Horizontal::Center),
                help_icon_with_tooltip
            ]
            .spacing(8);

            container(content)
                .width(self.width)
                .padding(self.padding)
                .style(move |_theme: &Theme| container::Style {
                    background: None,
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                    text_color: Some(colors.text),
                })
                .into()
        } else {
            // No help icon, just the label
            container(label)
                .width(self.width)
                .align_x(iced::alignment::Horizontal::Center)
                .padding(self.padding)
                .style(move |_theme: &Theme| container::Style {
                    background: None,
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                    text_color: Some(colors.text),
                })
                .into()
        }
    }
}

// Convenience function for creating the component
pub fn label_with_hint(
    label_text: impl Into<String>,
    hint_text: impl Into<String>,
) -> LabelWithHint {
    LabelWithHint::new(label_text, hint_text)
}

// Convenience function for creating themed component
pub fn themed_label_with_hint(
    label_text: impl Into<String>,
    hint_text: impl Into<String>,
    theme: NetMonkeyTheme,
) -> LabelWithHint {
    LabelWithHint::new(label_text, hint_text).theme(theme)
}
