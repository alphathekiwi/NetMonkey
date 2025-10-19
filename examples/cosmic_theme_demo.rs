//! COSMIC Theme Integration Demo
//!
//! This example demonstrates how to use Net Monkey's adaptive theme system
//! with COSMIC desktop integration.
//!
//! To run this example:
//! ```bash
//! # Standard mode (cross-platform)
//! cargo run --example cosmic_theme_demo
//!
//! # With COSMIC integration (Linux/COSMIC)
//! cargo run --example cosmic_theme_demo --features cosmic
//! ```

use iced::widget::{button, column, container, horizontal_space, row, text, vertical_space};
use iced::{Application, Element, Length, Settings, Task, Theme};
use std::collections::HashMap;

// Import theme system with conditional COSMIC support
use net_monkey_theme::{AdaptiveThemeManager, NetMonkeyColors, ThemeBackend};

#[cfg(feature = "cosmic")]
use net_monkey_theme::{
    cosmic_integration::{AdaptiveTheme, NetworkSpecificColors},
    is_cosmic_environment,
};

fn main() -> iced::Result {
    println!("🐒 Net Monkey COSMIC Theme Demo");

    #[cfg(feature = "cosmic")]
    {
        if is_cosmic_environment() {
            println!("🚀 COSMIC desktop environment detected!");
        } else {
            println!("📱 Non-COSMIC environment - will use fallback themes");
        }
    }

    #[cfg(not(feature = "cosmic"))]
    {
        println!("📦 Built without COSMIC integration");
    }

    ThemeDemo::run(Settings::default())
}

#[derive(Debug, Clone)]
pub enum Message {
    ThemeChanged(String),
    RefreshTheme,
    ShowNetworkColors,
    ToggleColorMode,
}

pub struct ThemeDemo {
    theme_manager: AdaptiveThemeManager,
    available_themes: Vec<String>,
    current_theme_name: String,
    show_network_colors: bool,
    network_colors: Option<NetworkColors>,
    color_mode_dark: bool,
}

// Simplified network colors for demo
#[derive(Debug, Clone)]
struct NetworkColors {
    online: (f32, f32, f32),
    offline: (f32, f32, f32),
    timeout: (f32, f32, f32),
    scanning: (f32, f32, f32),
    high_latency: (f32, f32, f32),
    low_latency: (f32, f32, f32),
}

impl Application for ThemeDemo {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Task<Message>) {
        let mut theme_manager = AdaptiveThemeManager::new();
        let available_themes = theme_manager.available_themes();
        let current_theme_name = theme_manager.name();
        let color_mode_dark = theme_manager.is_dark();

        let mut app = ThemeDemo {
            theme_manager,
            available_themes,
            current_theme_name,
            show_network_colors: false,
            network_colors: None,
            color_mode_dark,
        };

        app.update_network_colors();

        (app, Task::none())
    }

    fn title(&self) -> String {
        format!("COSMIC Theme Demo - {}", self.current_theme_name)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ThemeChanged(theme_name) => {
                if let Err(e) = self.theme_manager.switch_theme(&theme_name) {
                    eprintln!("Failed to switch theme: {}", e);
                } else {
                    self.current_theme_name = self.theme_manager.name();
                    self.color_mode_dark = self.theme_manager.is_dark();
                    self.update_network_colors();
                }
            }
            Message::RefreshTheme => {
                if let Err(e) = self.theme_manager.refresh() {
                    eprintln!("Failed to refresh theme: {}", e);
                } else {
                    self.current_theme_name = self.theme_manager.name();
                    self.color_mode_dark = self.theme_manager.is_dark();
                    self.update_network_colors();
                }
            }
            Message::ShowNetworkColors => {
                self.show_network_colors = !self.show_network_colors;
            }
            Message::ToggleColorMode => {
                // This is just for demo - in real app this would switch between light/dark
                println!(
                    "Color mode toggle requested (current: {})",
                    if self.color_mode_dark {
                        "dark"
                    } else {
                        "light"
                    }
                );
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let theme_info = self.build_theme_info();
        let theme_controls = self.build_theme_controls();
        let color_demos = self.build_color_demos();
        let network_colors = if self.show_network_colors {
            self.build_network_colors()
        } else {
            Element::from(text(
                "Click 'Show Network Colors' to see network-specific color palette",
            ))
        };

        let content = column![
            text("🐒 Net Monkey Theme Demo")
                .size(24)
                .style(|theme: &Theme| text::Style {
                    color: Some(theme.palette().primary),
                }),
            vertical_space().height(20),
            theme_info,
            vertical_space().height(20),
            theme_controls,
            vertical_space().height(20),
            text("Color Demonstrations:").size(18),
            color_demos,
            vertical_space().height(20),
            text("Network-Specific Colors:").size(18),
            network_colors,
        ]
        .spacing(10)
        .padding(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Theme {
        self.theme_manager.iced_theme()
    }
}

impl ThemeDemo {
    fn build_theme_info(&self) -> Element<Message> {
        let cosmic_status = self.get_cosmic_status();
        let theme_type = if self.color_mode_dark {
            "Dark"
        } else {
            "Light"
        };

        column![
            text(format!("Current Theme: {}", self.current_theme_name)),
            text(format!("Theme Type: {}", theme_type)),
            text(format!("COSMIC Status: {}", cosmic_status)),
            #[cfg(feature = "cosmic")]
            text(format!(
                "COSMIC Active: {}",
                self.theme_manager.is_cosmic_active()
            )),
        ]
        .spacing(5)
        .into()
    }

    fn build_theme_controls(&self) -> Element<Message> {
        let mut theme_buttons = row![];

        for theme_name in &self.available_themes {
            let is_current = theme_name == &self.current_theme_name;
            let button = button(text(theme_name))
                .on_press(Message::ThemeChanged(theme_name.clone()))
                .style(if is_current {
                    button::primary
                } else {
                    button::secondary
                });
            theme_buttons = theme_buttons
                .push(button)
                .push(horizontal_space().width(10));
        }

        column![
            text("Available Themes:"),
            theme_buttons,
            vertical_space().height(10),
            row![
                button("Refresh Theme").on_press(Message::RefreshTheme),
                horizontal_space().width(10),
                button("Toggle Color Mode").on_press(Message::ToggleColorMode),
                horizontal_space().width(10),
                button(if self.show_network_colors {
                    "Hide Network Colors"
                } else {
                    "Show Network Colors"
                })
                .on_press(Message::ShowNetworkColors),
            ]
        ]
        .spacing(10)
        .into()
    }

    fn build_color_demos(&self) -> Element<Message> {
        let colors = self.theme_manager.colors();

        let color_boxes = row![
            self.color_box("Primary", colors.primary),
            self.color_box("Success", colors.success),
            self.color_box("Warning", colors.warning),
            self.color_box("Danger", colors.danger),
        ]
        .spacing(15);

        let background_boxes = row![
            self.color_box("Background", colors.background),
            self.color_box("Menu", colors.menu),
            self.color_box("Sub Menu", colors.sub_menu),
        ]
        .spacing(15);

        column![
            text("Semantic Colors:"),
            color_boxes,
            vertical_space().height(10),
            text("Background Colors:"),
            background_boxes,
        ]
        .spacing(10)
        .into()
    }

    fn build_network_colors(&self) -> Element<Message> {
        if let Some(ref network_colors) = self.network_colors {
            let status_colors = row![
                self.rgb_color_box("Online", network_colors.online),
                self.rgb_color_box("Offline", network_colors.offline),
                self.rgb_color_box("Timeout", network_colors.timeout),
                self.rgb_color_box("Scanning", network_colors.scanning),
            ]
            .spacing(15);

            let performance_colors = row![
                self.rgb_color_box("High Latency", network_colors.high_latency),
                self.rgb_color_box("Low Latency", network_colors.low_latency),
            ]
            .spacing(15);

            column![
                text("Connection Status:"),
                status_colors,
                vertical_space().height(10),
                text("Performance Indicators:"),
                performance_colors,
                vertical_space().height(10),
                self.build_cosmic_info(),
            ]
            .spacing(10)
            .into()
        } else {
            text("Network colors not available").into()
        }
    }

    fn color_box(
        &self,
        label: &str,
        color: net_monkey_theme::SerializableColor,
    ) -> Element<Message> {
        let iced_color = iced::Color::from_rgba(color.r, color.g, color.b, color.a);

        column![
            container(text(""))
                .width(80)
                .height(50)
                .style(move |_theme: &Theme| {
                    container::Style {
                        background: Some(iced_color.into()),
                        border: iced::Border {
                            color: iced::Color::BLACK,
                            width: 1.0,
                            radius: 4.0.into(),
                        },
                        ..Default::default()
                    }
                }),
            text(label).size(12),
            text(format!(
                "#{:02X}{:02X}{:02X}",
                (color.r * 255.0) as u8,
                (color.g * 255.0) as u8,
                (color.b * 255.0) as u8
            ))
            .size(10),
        ]
        .spacing(5)
        .align_x(iced::alignment::Horizontal::Center)
        .into()
    }

    fn rgb_color_box(&self, label: &str, color: (f32, f32, f32)) -> Element<Message> {
        let iced_color = iced::Color::from_rgb(color.0, color.1, color.2);

        column![
            container(text(""))
                .width(80)
                .height(50)
                .style(move |_theme: &Theme| {
                    container::Style {
                        background: Some(iced_color.into()),
                        border: iced::Border {
                            color: iced::Color::BLACK,
                            width: 1.0,
                            radius: 4.0.into(),
                        },
                        ..Default::default()
                    }
                }),
            text(label).size(12),
            text(format!(
                "#{:02X}{:02X}{:02X}",
                (color.0 * 255.0) as u8,
                (color.1 * 255.0) as u8,
                (color.2 * 255.0) as u8
            ))
            .size(10),
        ]
        .spacing(5)
        .align_x(iced::alignment::Horizontal::Center)
        .into()
    }

    fn get_cosmic_status(&self) -> &'static str {
        #[cfg(feature = "cosmic")]
        {
            if is_cosmic_environment() {
                "Available"
            } else {
                "Not Available (Non-COSMIC Environment)"
            }
        }
        #[cfg(not(feature = "cosmic"))]
        {
            "Not Compiled (Missing 'cosmic' feature)"
        }
    }

    fn update_network_colors(&mut self) {
        #[cfg(feature = "cosmic")]
        {
            if self.theme_manager.is_cosmic_active() {
                // Use COSMIC's sophisticated network color derivation
                let adaptive = AdaptiveTheme::new();
                if let Ok(()) = adaptive.refresh_cosmic() {
                    let network_colors = adaptive.network_colors();
                    self.network_colors = Some(NetworkColors {
                        online: (
                            network_colors.online.r,
                            network_colors.online.g,
                            network_colors.online.b,
                        ),
                        offline: (
                            network_colors.offline.r,
                            network_colors.offline.g,
                            network_colors.offline.b,
                        ),
                        timeout: (
                            network_colors.timeout.r,
                            network_colors.timeout.g,
                            network_colors.timeout.b,
                        ),
                        scanning: (
                            network_colors.scanning.r,
                            network_colors.scanning.g,
                            network_colors.scanning.b,
                        ),
                        high_latency: (
                            network_colors.high_latency.r,
                            network_colors.high_latency.g,
                            network_colors.high_latency.b,
                        ),
                        low_latency: (
                            network_colors.low_latency.r,
                            network_colors.low_latency.g,
                            network_colors.low_latency.b,
                        ),
                    });
                    return;
                }
            }
        }

        // Fallback: use standard theme colors
        let colors = self.theme_manager.colors();
        self.network_colors = Some(NetworkColors {
            online: (colors.success.r, colors.success.g, colors.success.b),
            offline: (colors.danger.r, colors.danger.g, colors.danger.b),
            timeout: (colors.warning.r, colors.warning.g, colors.warning.b),
            scanning: (colors.primary.r, colors.primary.g, colors.primary.b),
            high_latency: (colors.danger.r, colors.danger.g, colors.danger.b),
            low_latency: (colors.success.r, colors.success.g, colors.success.b),
        });
    }

    fn build_cosmic_info(&self) -> Element<Message> {
        #[cfg(feature = "cosmic")]
        {
            if self.theme_manager.is_cosmic_active() {
                column![
                    text("🚀 Using COSMIC Theme Integration").style(|theme: &Theme| text::Style {
                        color: Some(theme.palette().success),
                    }),
                    text("Colors are derived using COSMIC's color science algorithms").size(12),
                    text("Theme will automatically update when system theme changes").size(12),
                ]
                .spacing(5)
                .into()
            } else {
                column![
                    text("📱 Using Standard Theme System").style(|theme: &Theme| text::Style {
                        color: Some(theme.palette().primary),
                    }),
                    text("Colors are manually defined in theme files").size(12),
                    text("Enable COSMIC integration by building with --features cosmic").size(12),
                ]
                .spacing(5)
                .into()
            }
        }
        #[cfg(not(feature = "cosmic"))]
        {
            column![
                text("📦 COSMIC Integration Not Available").style(|theme: &Theme| text::Style {
                    color: Some(theme.palette().danger),
                }),
                text("Build with --features cosmic to enable COSMIC integration").size(12),
                text("Currently using standard JSON-based themes").size(12),
            ]
            .spacing(5)
            .into()
        }
    }
}
