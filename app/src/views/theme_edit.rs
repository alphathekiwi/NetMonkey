use crate::Msg;
use iced::Alignment::Center;
use iced::Color;
use iced::Element;
use iced::Length::Fill;
use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced_widget::{horizontal_rule, pick_list};
use net_monkey_theme::{NetMonkeyColors, NetMonkeyTheme};

use crate::views::settings::{ColorType, IpScannerApp};

pub fn view<'a>(app: &'a IpScannerApp) -> Element<'a, Msg> {
    // Get colors from current theme
    let current_colors = app.config.theme.clone().colors();

    // Theme selector
    let theme_options = NetMonkeyTheme::all();

    let theme_selector = row![
        text("Base Theme:").size(18).width(120),
        pick_list(theme_options, Some(app.config.theme.clone()), |theme| {
            Msg::Config(crate::views::settings::ChangeConfig::Theme(theme))
        })
        .text_size(16)
        .width(200),
    ]
    .spacing(10)
    .align_y(Center);

    // Color editing sections
    let background_section = create_color_section(
        "Background Colors",
        &[
            (
                "Background",
                current_colors.background.into(),
                ColorType::Background,
            ),
            ("Menu", current_colors.menu.into(), ColorType::Menu),
            (
                "Sub Menu",
                current_colors.sub_menu.into(),
                ColorType::SubMenu,
            ),
        ],
    );

    let text_section = create_color_section(
        "Text Colors",
        &[
            ("Primary Text", current_colors.text.into(), ColorType::Text),
            (
                "Secondary Text",
                current_colors.text_secondary.into(),
                ColorType::TextSecondary,
            ),
        ],
    );

    let accent_section = create_color_section(
        "Accent Colors",
        &[
            ("Primary", current_colors.primary.into(), ColorType::Primary),
            ("Success", current_colors.success.into(), ColorType::Success),
            ("Warning", current_colors.warning.into(), ColorType::Warning),
            ("Danger", current_colors.danger.into(), ColorType::Danger),
        ],
    );

    let border_section = create_color_section(
        "Border Colors",
        &[
            ("Border", current_colors.border.into(), ColorType::Border),
            (
                "Focused Border",
                current_colors.border_focused.into(),
                ColorType::BorderFocused,
            ),
            (
                "Hover Border",
                current_colors.border_hover.into(),
                ColorType::BorderHover,
            ),
            (
                "Disabled Border",
                current_colors.border_disabled.into(),
                ColorType::BorderDisabled,
            ),
        ],
    );

    let state_section = create_color_section(
        "State Colors",
        &[
            ("Active", current_colors.active.into(), ColorType::Active),
            ("Hover", current_colors.hover.into(), ColorType::Hover),
        ],
    );

    // Action buttons
    let actions = row![
        button(text("Reset to Base Theme").center())
            .on_press(Msg::Config(crate::views::settings::ChangeConfig::Theme(
                app.config.theme.clone()
            )))
            .style(button::secondary)
            .padding(8)
            .width(Fill),
        button(text("Save as New Theme").center())
            .on_press(Msg::SaveTheme) // New message for saving themes
            .style(button::primary)
            .padding(8)
            .width(Fill),
        button(text("Reload Themes").center())
            .on_press(Msg::Config(crate::views::settings::ChangeConfig::Theme(
                app.config.theme.clone()
            )))
            .style(button::text)
            .padding(8)
            .width(Fill),
    ]
    .spacing(8);

    // Theme indicator showing current theme name
    let theme_name = app.config.theme.name();
    let display_name = if theme_name.starts_with("editing_") {
        "Custom Theme (Editing)".to_string()
    } else {
        format!("Editing: {theme_name}")
    };

    let theme_status = container(text(display_name).size(14))
        .style(|_| container::Style {
            background: Some(iced::Background::Color(Color::from_rgba(
                0.2, 0.4, 0.8, 0.2,
            ))),
            border: iced::Border {
                color: Color::from_rgba(0.2, 0.4, 0.8, 0.5),
                width: 1.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        })
        .padding(8)
        .center_x(Fill);

    let content = column![
        text("Theme Editor").size(24),
        theme_status,
        horizontal_rule(2),
        theme_selector,
        horizontal_rule(1),
        background_section,
        text_section,
        accent_section,
        border_section,
        state_section,
        horizontal_rule(1),
        text("Preview").size(20),
        create_theme_preview(current_colors),
        horizontal_rule(1),
        actions,
    ]
    .align_x(Center)
    .spacing(15)
    .padding(20);

    scrollable(content)
        .height(Fill)
        .direction(iced::widget::scrollable::Direction::Vertical(
            iced::widget::scrollable::Scrollbar::new()
                .width(8)
                .scroller_width(8),
        ))
        .into()
}

fn create_color_section<'a>(
    title: &'a str,
    colors: &[(&'a str, Color, ColorType)],
) -> Element<'a, Msg> {
    let title_element = text(title).size(18);

    let color_rows: Vec<Element<'a, Msg>> = colors
        .iter()
        .map(|(name, color, color_type)| create_color_row(name, *color, color_type.clone()))
        .collect();

    let mut column_content = vec![title_element.into()];
    column_content.extend(color_rows);

    container(column(column_content).spacing(8).padding(10))
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(Color::from_rgba(
                0.0, 0.0, 0.0, 0.1,
            ))),
            border: iced::Border {
                color: Color::from_rgba(0.5, 0.5, 0.5, 0.3),
                width: 1.0,
                radius: 8.0.into(),
            },
            ..container::Style::default()
        })
        .width(Fill)
        .into()
}

fn create_color_row<'a>(name: &'a str, color: Color, color_type: ColorType) -> Element<'a, Msg> {
    let color_hex = color_to_hex(color);

    let color_preview = container(text(""))
        .width(40)
        .height(25)
        .style(move |_| container::Style {
            background: Some(iced::Background::Color(color)),
            border: iced::Border {
                color: Color::BLACK,
                width: 1.0,
                radius: 4.0.into(),
            },
            ..container::Style::default()
        });

    let color_input = text_input("", &color_hex)
        .on_input(move |hex_value| Msg::ColorEdit {
            color_type: color_type.clone(),
            hex_value,
        })
        .size(14)
        .width(100);

    row![text(name).size(14).width(120), color_preview, color_input,]
        .spacing(10)
        .align_y(Center)
        .into()
}

fn create_theme_preview(colors: NetMonkeyColors) -> Element<'static, Msg> {
    let preview_content = column![
        container(text("Sample Menu Item").color(Color::from(colors.text)))
            .style(move |_| container::Style {
                background: Some(iced::Background::Color(colors.menu.into())),
                border: iced::Border {
                    color: colors.border.into(),
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..container::Style::default()
            })
            .padding(10)
            .width(Fill),
        container(text("Sample Sub-Menu Item").color(Color::from(colors.text)))
            .style(move |_| container::Style {
                background: Some(iced::Background::Color(colors.sub_menu.into())),
                border: iced::Border {
                    color: colors.border_focused.into(),
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..container::Style::default()
            })
            .padding(8)
            .width(Fill),
        row![
            button(text("Primary Button").color(Color::from(colors.text)))
                .style(move |_theme, _status| button::Style {
                    background: Some(iced::Background::Color(colors.primary.into())),
                    text_color: colors.text.into(),
                    border: iced::Border {
                        color: colors.border_focused.into(),
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    ..button::Style::default()
                })
                .padding(8),
            button(text("Success Button").color(Color::from(colors.text)))
                .style(move |_theme, _status| button::Style {
                    background: Some(iced::Background::Color(colors.success.into())),
                    text_color: colors.text.into(),
                    border: iced::Border {
                        color: colors.border.into(),
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    ..button::Style::default()
                })
                .padding(8),
            button(text("Danger Button").color(Color::from(colors.text)))
                .style(move |_theme, _status| button::Style {
                    background: Some(iced::Background::Color(colors.danger.into())),
                    text_color: colors.text.into(),
                    border: iced::Border {
                        color: colors.border.into(),
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    ..button::Style::default()
                })
                .padding(8),
        ]
        .spacing(10),
        text("Secondary text example")
            .color(Color::from(colors.text_secondary))
            .size(14),
        text("Warning text example")
            .color(Color::from(colors.warning))
            .size(14),
    ]
    .spacing(10);

    container(preview_content)
        .style(move |_| container::Style {
            background: Some(iced::Background::Color(colors.background.into())),
            border: iced::Border {
                color: colors.border.into(),
                width: 2.0,
                radius: 8.0.into(),
            },
            ..container::Style::default()
        })
        .padding(15)
        .width(Fill)
        .into()
}

fn color_to_hex(color: Color) -> String {
    let r = (color.r * 255.0) as u8;
    let g = (color.g * 255.0) as u8;
    let b = (color.b * 255.0) as u8;
    format!("#{r:02X}{g:02X}{b:02X}")
}

// Helper function to parse hex color (for future use when implementing color input)
#[allow(dead_code)]
fn hex_to_color(hex: &str) -> Option<Color> {
    if !hex.starts_with('#') || hex.len() != 7 {
        return None;
    }

    let hex = &hex[1..];
    if let (Ok(r), Ok(g), Ok(b)) = (
        u8::from_str_radix(&hex[0..2], 16),
        u8::from_str_radix(&hex[2..4], 16),
        u8::from_str_radix(&hex[4..6], 16),
    ) {
        Some(Color::from_rgb(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
        ))
    } else {
        None
    }
}
