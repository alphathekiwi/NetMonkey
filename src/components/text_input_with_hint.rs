use crate::theme::NetMonkeyTheme;
use iced::widget::{container, row, text, text_input, tooltip};
use iced::{Color, Element, Length, Padding, Renderer, Theme};

/// A text input component with an optional help hint icon that shows a tooltip on hover.
///
/// This component combines a standard text input field with a circular help icon
/// that displays hint text when hovered over. The help icon uses a simple container-based
/// design with NetMonkey theming integration for consistent styling across the application.
///
/// # Features
/// - Standard text input functionality
/// - Simple help hint icon with circle and "?" symbol that inherits text size
/// - Tooltip display on hover using iced's built-in tooltip widget
/// - Full NetMonkey theme integration with automatic color adaptation
/// - Support for Dark, Light, and High Contrast themes
/// - Chainable builder pattern for configuration
/// - Professional monitoring application aesthetic
///
/// # Examples
///
/// ## Basic Usage
/// ```rust
/// let input = TextInputWithHint::new(
///     current_value,
///     "Enter IP address",
///     "Format: xxx.xxx.xxx.xxx (e.g., 192.168.1.1)",
///     |text| Message::IpChanged(text)
/// );
/// ```
///
/// ## Customized Usage with Theming
/// ```rust
/// let input = TextInputWithHint::new(
///     port_value,
///     "Port number",
///     "Valid port range: 1-65535.\nCommon ports: 80 (HTTP), 443 (HTTPS), 22 (SSH)",
///     |text| Message::PortChanged(text)
/// )
/// .width(Length::Fixed(200.0))
/// .text_size(16.0)
/// .theme(NetMonkeyTheme::Light);
/// ```
pub struct TextInputWithHint<'a, Message> {
    value: String,
    placeholder: String,
    hint_text: String,
    on_input: Box<dyn Fn(String) -> Message + 'a>,
    width: Length,
    text_size: f32,
    padding: Padding,
    theme: NetMonkeyTheme,
}

impl<'a, Message> TextInputWithHint<'a, Message>
where
    Message: Clone + 'a,
{
    /// Creates a new TextInputWithHint component
    ///
    /// # Arguments
    /// * `value` - Current text value
    /// * `placeholder` - Placeholder text for the input field
    /// * `hint_text` - Help text to show in tooltip
    /// * `on_input` - Callback function for text changes
    pub fn new<F>(
        value: String,
        placeholder: impl Into<String>,
        hint_text: impl Into<String>,
        on_input: F,
    ) -> Self
    where
        F: Fn(String) -> Message + 'a,
    {
        Self {
            value,
            placeholder: placeholder.into(),
            hint_text: hint_text.into(),
            on_input: Box::new(on_input),
            width: Length::Fill,
            text_size: 14.0,
            padding: Padding::new(8.0),
            theme: NetMonkeyTheme::Dark,
        }
    }

    /// Sets the width of the component
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the text size for the input field and help icon
    pub fn text_size(mut self, size: f32) -> Self {
        self.text_size = size;
        self
    }

    /// Sets the padding around the component
    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    /// Sets the NetMonkey theme for the component
    ///
    /// This applies the appropriate color scheme including:
    /// - Input field styling that adapts to the theme
    /// - Help icon background and border colors
    /// - Tooltip background and text colors
    /// - Container border colors that match the theme
    ///
    /// # Arguments
    /// * `theme` - The NetMonkey theme variant to apply
    pub fn theme(mut self, theme: NetMonkeyTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Converts the component into an Element
    pub fn into_element(self) -> Element<'a, Message, Theme, Renderer> {
        let colors = self.theme.colors();

        let input = text_input(&self.placeholder, &self.value)
            .on_input(self.on_input)
            .size(self.text_size)
            .width(Length::Fill);

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
                tooltip::Position::Right,
            );

            let content = row![input, help_icon_with_tooltip].spacing(8);

            container(content)
                .width(self.width)
                .padding(self.padding)
                .style(move |_theme: &Theme| container::Style {
                    background: Some(iced::Background::Color(colors.background)),
                    border: iced::Border {
                        color: colors.border,
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    text_color: Some(colors.text),
                    shadow: iced::Shadow::default(),
                })
                .into()
        } else {
            // No help icon, just the input field
            container(input)
                .width(self.width)
                .padding(self.padding)
                .style(move |_theme: &Theme| container::Style {
                    background: Some(iced::Background::Color(colors.background)),
                    border: iced::Border {
                        color: colors.border,
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    text_color: Some(colors.text),
                    shadow: iced::Shadow::default(),
                })
                .into()
        }
    }
}

// Convenience function for creating the component
pub fn text_input_with_hint<'a, Message>(
    value: String,
    placeholder: impl Into<String>,
    hint_text: impl Into<String>,
    on_input: impl Fn(String) -> Message + 'a,
) -> TextInputWithHint<'a, Message>
where
    Message: Clone + 'a,
{
    TextInputWithHint::new(value, placeholder, hint_text, on_input)
}

// Convenience function for creating themed component
pub fn themed_text_input_with_hint<'a, Message>(
    value: String,
    placeholder: impl Into<String>,
    hint_text: impl Into<String>,
    on_input: impl Fn(String) -> Message + 'a,
    theme: NetMonkeyTheme,
) -> TextInputWithHint<'a, Message>
where
    Message: Clone + 'a,
{
    TextInputWithHint::new(value, placeholder, hint_text, on_input).theme(theme)
}
