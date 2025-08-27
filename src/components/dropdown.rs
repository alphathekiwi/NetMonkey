use std::borrow::Borrow;

use iced::Pixels;

use iced::widget::pick_list::Catalog;
use iced_core::clipboard::Clipboard;
use iced_core::event::{self, Event};
use iced_core::keyboard;
use iced_core::layout::{self, Layout};
use iced_core::mouse;
use iced_core::overlay;
use iced_core::renderer;
use iced_core::text::{self};
use iced_core::widget::{self, Tree, Widget};
use iced_core::{Border, Color, Length, Padding, Rectangle, Size, Vector};
use iced_widget::graphics::text::cosmic_text::ttf_parser::ankr::Point;
use iced_widget::text_input::Status;

/// A text input field with a dropdown button for selecting from predefined options.
///
/// This component allows users to either type directly into the text field or select
/// from a dropdown list of predefined items.
///
/// # Example
/// ```rust
/// let items = vec!["192.168.1.1", "10.0.0.1", "127.0.0.1"];
/// let dropdown = TextInputDropdown::new(
///     items,
///     current_value,
///     |text| Message::TextChanged(text),
///     |selected| Message::ItemSelected(selected),
/// )
/// .placeholder("Enter or select an IP...")
/// .size(24.into())
/// .width(Length::Fixed(300.0));
/// ```
pub struct TextInputDropdown<'a, T, L, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    T: ToString + PartialEq + Clone,
    L: Borrow<[T]> + 'a + std::fmt::Debug,
    Theme: Catalog + iced::widget::text_input::Catalog + iced::widget::button::Catalog,
    Renderer: text::Renderer,
{
    on_input: Box<dyn Fn(String) -> Message + 'a>,
    on_select: Box<dyn Fn(T) -> Message + 'a>,
    on_submit: Option<Message>,
    value: String,
    items: L,
    placeholder: Option<String>,
    width: Length,
    padding: Padding,
    text_size: Option<Pixels>,
    text_line_height: text::LineHeight,
    text_shaping: text::Shaping,
    font: Option<Renderer::Font>,
    class: <Theme as Catalog>::Class<'a>,
}

impl<'a, T, L, Message, Theme, Renderer> TextInputDropdown<'a, T, L, Message, Theme, Renderer>
where
    T: ToString + PartialEq + Clone,
    L: Borrow<[T]> + 'a + std::fmt::Debug,
    Theme: Catalog + iced::widget::text_input::Catalog + iced::widget::button::Catalog,
    Renderer: text::Renderer,
{
    pub fn new<F1, F2>(items: L, value: String, on_input: F1, on_select: F2) -> Self
    where
        F1: Fn(String) -> Message + 'a,
        F2: Fn(T) -> Message + 'a,
    {
        Self {
            on_input: Box::new(on_input),
            on_select: Box::new(on_select),
            on_submit: None,
            value,
            items,
            placeholder: None,
            width: Length::Fill,
            padding: Padding::new(8.0),
            text_size: None,
            text_line_height: text::LineHeight::default(),
            text_shaping: text::Shaping::default(),
            font: None,
            class: <Theme as Catalog>::default(),
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn text_size(mut self, size: Pixels) -> Self {
        self.text_size = Some(size);
        self
    }

    pub fn on_submit(mut self, message: Message) -> Self {
        self.on_submit = Some(message);
        self
    }

    pub fn size(mut self, size: Pixels) -> Self {
        self.text_size = Some(size);
        self
    }
}

impl<'a, T, L, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for TextInputDropdown<'a, T, L, Message, Theme, Renderer>
where
    T: ToString + PartialEq + Clone,
    L: Borrow<[T]> + 'a + std::fmt::Debug,
    Theme: Catalog + iced::widget::text_input::Catalog + iced::widget::button::Catalog,
    Renderer: text::Renderer,
    Message: Clone,
{
    fn tag(&self) -> widget::tree::Tag {
        widget::tree::Tag::of::<State>()
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(State::new())
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut iced_core::Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> event::Status {
        let state: &mut State = tree.state.downcast_mut();
        let bounds = layout.bounds();

        // Button bounds (right side of the widget)
        let button_width = 30.0;
        let button_bounds = Rectangle {
            x: bounds.x + bounds.width - button_width,
            y: bounds.y,
            width: button_width,
            height: bounds.height,
        };

        // Text input bounds (left side of the widget)
        let input_bounds = Rectangle {
            x: bounds.x,
            y: bounds.y,
            width: bounds.width - button_width,
            height: bounds.height,
        };

        match &event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if let Some(position) = cursor.position() {
                    if button_bounds.contains(position) {
                        state.is_open = !state.is_open;
                        state.is_focused = true;
                        return event::Status::Captured;
                    } else if input_bounds.contains(position) {
                        state.is_focused = true;
                        state.is_open = false;
                        // Update cursor position
                        let relative_x = position.x - input_bounds.x - self.padding.left;
                        state.cursor_position = self.cursor_position_from_x(relative_x, renderer);
                        return event::Status::Captured;
                    } else if state.is_open {
                        // Check if clicking on dropdown items
                        let dropdown_y = bounds.y + bounds.height;
                        let item_height = 30.0;
                        for (index, item) in self.items.borrow().iter().enumerate() {
                            let item_y = dropdown_y + (index as f32 * item_height);
                            let item_bounds = Rectangle {
                                x: bounds.x,
                                y: item_y,
                                width: bounds.width,
                                height: item_height,
                            };

                            if item_bounds.contains(position) {
                                shell.publish((self.on_select)(item.clone()));
                                state.is_open = false;
                                state.is_focused = false;
                                return event::Status::Captured;
                            }
                        }
                        // Click outside dropdown - close it
                        state.is_open = false;
                        state.is_focused = false;
                    } else {
                        state.is_open = false;
                        state.is_focused = false;
                    }
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                if state.is_open {
                    let dropdown_y = bounds.y + bounds.height;
                    let item_height = 30.0;
                    state.hovered_option = None;

                    for (index, _) in self.items.borrow().iter().enumerate() {
                        let item_y = dropdown_y + (index as f32 * item_height);
                        let item_bounds = Rectangle {
                            x: bounds.x,
                            y: item_y,
                            width: bounds.width,
                            height: item_height,
                        };

                        if item_bounds.contains(*position) {
                            state.hovered_option = Some(index);
                            break;
                        }
                    }
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) => {
                if state.is_focused {
                    state.keyboard_modifiers = *modifiers;
                    match key {
                        keyboard::Key::Character(c) => {
                            if !modifiers.control() && !modifiers.logo() {
                                let char_str = c.to_string();
                                self.value.insert_str(state.cursor_position, &char_str);
                                state.cursor_position += char_str.len();
                                shell.publish((self.on_input)(self.value.clone()));
                                return event::Status::Captured;
                            }
                        }
                        keyboard::Key::Named(keyboard::key::Named::Backspace) => {
                            if state.cursor_position > 0 {
                                let prev_cursor = state.cursor_position;
                                state.cursor_position = prev_cursor.saturating_sub(1);
                                if state.cursor_position < self.value.len() {
                                    self.value.remove(state.cursor_position);
                                }
                                shell.publish((self.on_input)(self.value.clone()));
                                return event::Status::Captured;
                            }
                        }
                        keyboard::Key::Named(keyboard::key::Named::Delete) => {
                            if state.cursor_position < self.value.len() {
                                self.value.remove(state.cursor_position);
                                shell.publish((self.on_input)(self.value.clone()));
                                return event::Status::Captured;
                            }
                        }
                        keyboard::Key::Named(keyboard::key::Named::ArrowLeft) => {
                            state.cursor_position = state.cursor_position.saturating_sub(1);
                            return event::Status::Captured;
                        }
                        keyboard::Key::Named(keyboard::key::Named::ArrowRight) => {
                            state.cursor_position =
                                (state.cursor_position + 1).min(self.value.len());
                            return event::Status::Captured;
                        }
                        keyboard::Key::Named(keyboard::key::Named::ArrowDown) => {
                            if !state.is_open {
                                state.is_open = true;
                                return event::Status::Captured;
                            }
                        }
                        keyboard::Key::Named(keyboard::key::Named::Enter) => {
                            if let Some(ref message) = self.on_submit {
                                shell.publish(message.clone());
                                return event::Status::Captured;
                            }
                        }
                        keyboard::Key::Named(keyboard::key::Named::Escape) => {
                            if state.is_open {
                                state.is_open = false;
                                return event::Status::Captured;
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        event::Status::Ignored
    }

    fn overlay<'b>(
        &'b mut self,
        _tree: &'b mut Tree,
        _layout: Layout<'_>,
        _renderer: &Renderer,
        _translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        // For now, we'll handle dropdown rendering in the draw method
        // to avoid lifetime issues with Menu overlay
        None
    }

    fn size(&self) -> Size<Length> {
        Size::new(self.width, Length::Shrink)
    }

    fn layout(
        &self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let text_size = self.text_size.unwrap_or(Pixels(14.0));
        let line_height = match self.text_line_height {
            text::LineHeight::Absolute(pixels) => pixels.0,
            text::LineHeight::Relative(factor) => text_size.0 * factor,
        };

        let content_height = line_height.max(text_size.0 * 1.2);
        let height = content_height + self.padding.vertical();
        let height = height.max(32.0);

        let size = limits.resolve(self.width, Length::Fixed(height.into()), Size::ZERO);

        layout::Node::new(size)
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let state: &State = tree.state.downcast_ref();
        let bounds = layout.bounds();
        let text_size = self.text_size.unwrap_or(Pixels(14.0));
        let font = self.font.unwrap_or_else(|| renderer.default_font());

        // Button bounds (right side)
        let button_width = 30.0;
        let button_bounds = Rectangle {
            x: bounds.x + bounds.width - button_width,
            y: bounds.y,
            width: button_width,
            height: bounds.height,
        };

        // Text input bounds (left side of the widget)
        let input_bounds = Rectangle {
            x: bounds.x,
            y: bounds.y,
            width: bounds.width - button_width,
            height: bounds.height,
        };
        let is_mouse_over = cursor.is_over(bounds);

        let text_input_status = if self.items.borrow().is_empty() {
            Status::Disabled
        } else if state.is_focused || state.is_open {
            Status::Focused
        } else if is_mouse_over {
            Status::Hovered
        } else {
            Status::Active
        };

        let pick_list_status = match text_input_status {
            Status::Disabled => iced::widget::pick_list::Status::Active,
            Status::Focused => iced::widget::pick_list::Status::Active,
            Status::Hovered => iced::widget::pick_list::Status::Hovered,
            Status::Active => iced::widget::pick_list::Status::Active,
        };

        let pick_list_style = <Theme as Catalog>::style(theme, &self.class, pick_list_status);

        // Draw text input background
        let input_appearance = iced::widget::text_input::Style {
            background: pick_list_style.background,
            border: Border {
                color: if state.is_focused {
                    Color::from_rgb(0.0, 0.5, 1.0)
                } else {
                    Color::from_rgb(0.7, 0.7, 0.7)
                },
                width: 1.0,
                radius: iced::border::Radius::new(4.0),
            },
            icon: pick_list_style.text_color,
            value: pick_list_style.text_color,
            placeholder: Color::from_rgb(0.5, 0.5, 0.5),
            selection: Color::from_rgb(0.0, 0.5, 1.0),
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds: input_bounds,
                border: Border {
                    color: input_appearance.border.color,
                    width: input_appearance.border.width,
                    radius: input_appearance.border.radius,
                },
                shadow: iced_core::Shadow::default(),
            },
            input_appearance.background,
        );

        // Draw text
        println!("{:?}\n{:?}", input_bounds, self.padding);
        let text_bounds = Rectangle {
            x: input_bounds.x + self.padding.left,
            y: input_bounds.y + self.padding.top,
            width: input_bounds.width - self.padding.horizontal(),
            height: input_bounds.height - self.padding.vertical(),
        };

        let display_text = if self.value.is_empty() {
            self.placeholder.as_deref().unwrap_or("")
        } else {
            &self.value
        };

        let text_color = if self.value.is_empty() && self.placeholder.is_some() {
            Color::from_rgb(0.5, 0.5, 0.5) // Placeholder color
        } else {
            input_appearance.value
        };

        renderer.fill_text(
            text::Text {
                content: display_text.to_string(),
                size: text_size,
                line_height: self.text_line_height,
                font,
                bounds: text_bounds.size(),
                horizontal_alignment: iced::alignment::Horizontal::Left,
                vertical_alignment: iced::alignment::Vertical::Center,
                shaping: self.text_shaping,
                wrapping: text::Wrapping::None,
            },
            text_bounds.position()
                + Vector {
                    x: 0.0,
                    y: text_bounds.height / 2.0,
                },
            text_color,
            text_bounds,
        );

        // Draw cursor if focused
        if state.is_focused && !self.value.is_empty() {
            let cursor_x = self.cursor_x_position(state.cursor_position, renderer);
            let cursor_bounds = Rectangle {
                x: text_bounds.x + cursor_x,
                y: text_bounds.y + 2.0,
                width: 1.0,
                height: text_bounds.height - 4.0,
            };

            renderer.fill_quad(
                renderer::Quad {
                    bounds: cursor_bounds,
                    border: Border::default(),
                    shadow: iced_core::Shadow::default(),
                },
                iced_core::Background::Color(text_color),
            );
        }

        // Draw dropdown button
        let button_style = iced::widget::button::Style {
            background: Some(iced_core::Background::Color(Color::from_rgb(0.9, 0.9, 0.9))),
            text_color: Color::from_rgb(0.3, 0.3, 0.3),
            border: Border {
                color: Color::from_rgb(0.7, 0.7, 0.7),
                width: 1.0,
                radius: iced::border::Radius::new(4.0),
            },
            shadow: iced_core::Shadow::default(),
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds: button_bounds,
                border: button_style.border,
                shadow: button_style.shadow,
            },
            button_style.background.unwrap(),
        );

        // Draw arrow
        let arrow_size = text_size.0;
        let arrow_x = button_bounds.x + (button_bounds.width - arrow_size) / 2.0;
        let arrow_y = button_bounds.y + (button_bounds.height - arrow_size) / 2.0;

        // Simple triangle arrow pointing down
        renderer.fill_text(
            text::Text {
                content: "▼".to_string(),
                size: Pixels(arrow_size),
                line_height: text::LineHeight::default(),
                font,
                bounds: Size::new(arrow_size, arrow_size),
                horizontal_alignment: iced::alignment::Horizontal::Center,
                vertical_alignment: iced::alignment::Vertical::Center,
                shaping: text::Shaping::default(),
                wrapping: text::Wrapping::None,
            },
            button_bounds.center(),
            button_style.text_color,
            Rectangle::new(
                iced_core::Point::new(arrow_x, arrow_y),
                Size::new(arrow_size, arrow_size),
            ),
        );

        // Draw simple dropdown list if open
        if state.is_open {
            let dropdown_y = bounds.y + bounds.height;
            let dropdown_height = (self.items.borrow().len() as f32 * 30.0).min(200.0);
            let dropdown_bounds = Rectangle {
                x: bounds.x,
                y: dropdown_y,
                width: bounds.width,
                height: dropdown_height,
            };

            // Draw dropdown background
            renderer.fill_quad(
                renderer::Quad {
                    bounds: dropdown_bounds,
                    border: Border {
                        color: Color::from_rgb(0.7, 0.7, 0.7),
                        width: 1.0,
                        radius: iced::border::Radius::new(4.0),
                    },
                    shadow: iced_core::Shadow::default(),
                },
                iced_core::Background::Color(Color::WHITE),
            );

            // Draw dropdown items
            let item_height = 30.0;
            for (index, item) in self.items.borrow().iter().enumerate() {
                let item_y = dropdown_y + (index as f32 * item_height);
                let item_bounds = Rectangle {
                    x: bounds.x,
                    y: item_y,
                    width: bounds.width,
                    height: item_height,
                };

                // Highlight hovered item
                let is_hovered = state.hovered_option == Some(index);
                if is_hovered {
                    renderer.fill_quad(
                        renderer::Quad {
                            bounds: item_bounds,
                            border: Border::default(),
                            shadow: iced_core::Shadow::default(),
                        },
                        iced_core::Background::Color(Color::from_rgb(0.9, 0.95, 1.0)),
                    );
                }

                // Draw item text
                renderer.fill_text(
                    text::Text {
                        content: item.to_string(),
                        size: text_size,
                        line_height: self.text_line_height,
                        font,
                        bounds: item_bounds.size(),
                        horizontal_alignment: iced::alignment::Horizontal::Left,
                        vertical_alignment: iced::alignment::Vertical::Center,
                        shaping: self.text_shaping,
                        wrapping: text::Wrapping::None,
                    },
                    iced_core::Point::new(item_bounds.x + self.padding.left, item_bounds.y),
                    Color::BLACK,
                    item_bounds,
                );
            }
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let bounds = layout.bounds();
        let button_width = 30.0;
        let button_bounds = Rectangle {
            x: bounds.x + bounds.width - button_width,
            y: bounds.y,
            width: button_width,
            height: bounds.height,
        };

        if cursor.is_over(button_bounds) || cursor.is_over(bounds) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }
}

struct State {
    keyboard_modifiers: keyboard::Modifiers,
    is_open: bool,
    is_focused: bool,
    cursor_position: usize,
    hovered_option: Option<usize>,
}

impl State {
    fn new() -> Self {
        Self {
            keyboard_modifiers: keyboard::Modifiers::default(),
            is_open: false,
            is_focused: false,
            cursor_position: 0,
            hovered_option: None,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T, L, Message, Theme, Renderer> TextInputDropdown<'a, T, L, Message, Theme, Renderer>
where
    T: ToString + PartialEq + Clone,
    L: Borrow<[T]> + 'a + std::fmt::Debug,
    Theme: Catalog + iced::widget::text_input::Catalog + iced::widget::button::Catalog,
    Renderer: text::Renderer,
{
    fn cursor_x_position(&self, cursor_position: usize, _renderer: &Renderer) -> f32 {
        if cursor_position == 0 || self.value.is_empty() {
            return 0.0;
        }

        let text_before_cursor = &self.value[..cursor_position.min(self.value.len())];
        let text_size = self.text_size.unwrap_or(Pixels(14.0));

        // Simple approximation for cursor position
        text_before_cursor.len() as f32 * (text_size.0 * 0.6)
    }

    fn cursor_position_from_x(&self, x: f32, _renderer: &Renderer) -> usize {
        if self.value.is_empty() || x <= 0.0 {
            return 0;
        }

        let text_size = self.text_size.unwrap_or(Pixels(14.0));
        let char_width = text_size.0 * 0.6; // Approximation
        let position = (x / char_width) as usize;
        std::cmp::min(position, self.value.len())
    }
}

impl<'a, T, L, Message, Theme, Renderer> From<TextInputDropdown<'a, T, L, Message, Theme, Renderer>>
    for iced::Element<'a, Message, Theme, Renderer>
where
    T: ToString + PartialEq + Clone + 'a,
    L: Borrow<[T]> + 'a + std::fmt::Debug,
    Theme: Catalog + iced::widget::text_input::Catalog + iced::widget::button::Catalog + 'a,
    Renderer: text::Renderer + 'a,
    Message: Clone + 'a,
{
    fn from(dropdown: TextInputDropdown<'a, T, L, Message, Theme, Renderer>) -> Self {
        iced::Element::new(dropdown)
    }
}
