use std::borrow::Borrow;

use iced::Pixels;
use iced::overlay::menu::{self, Menu};
use iced::widget::pick_list::{Catalog, Handle};
use iced_core::clipboard::Clipboard;
use iced_core::event::{self, Event};
use iced_core::keyboard;
use iced_core::layout::{self, Layout};
use iced_core::mouse;
use iced_core::overlay;
use iced_core::renderer;
use iced_core::text::{self, paragraph};
use iced_core::widget::{self, Tree, Widget};
use iced_core::{Border, Color, Length, Padding, Rectangle, Size, Vector};

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
    L: Borrow<[T]> + 'a,
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
    handle: Handle<Renderer::Font>,
    class: <Theme as Catalog>::Class<'a>,
    menu_class: <Theme as menu::Catalog>::Class<'a>,
}

impl<'a, T, L, Message, Theme, Renderer> TextInputDropdown<'a, T, L, Message, Theme, Renderer>
where
    T: ToString + PartialEq + Clone,
    L: Borrow<[T]> + 'a,
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
            handle: Handle::Arrow { size: None },
            class: <Theme as Catalog>::default(),
            menu_class: <Theme as menu::Catalog>::default(),
        }
    }

    pub fn placeholder<S: Into<String>>(mut self, placeholder: S) -> Self {
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
    T: Clone + ToString + PartialEq + 'a,
    L: Borrow<[T]>,
    Message: Clone + 'a,
    Theme: Catalog + iced::widget::text_input::Catalog + iced::widget::button::Catalog + 'a,
    Renderer: text::Renderer + 'a,
{
    fn tag(&self) -> widget::tree::Tag {
        widget::tree::Tag::of::<State<Renderer::Paragraph>>()
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(State::<Renderer::Paragraph>::new())
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
        let state = tree.state.downcast_mut::<State<Renderer::Paragraph>>();
        let bounds = layout.bounds();

        // Calculate button bounds (rightmost 32px)
        let button_width = 32.0;
        let button_bounds = Rectangle {
            x: bounds.x + bounds.width - button_width,
            y: bounds.y,
            width: button_width,
            height: bounds.height,
        };

        // Calculate text input bounds (remaining space)
        let input_bounds = Rectangle {
            x: bounds.x,
            y: bounds.y,
            width: bounds.width - button_width,
            height: bounds.height,
        };

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if let Some(cursor_position) = cursor.position() {
                    if button_bounds.contains(cursor_position) {
                        // Toggle dropdown
                        state.is_open = !state.is_open;
                        state.is_focused = false;
                        return event::Status::Captured;
                    } else if input_bounds.contains(cursor_position) {
                        // Focus text input
                        state.is_focused = true;
                        state.is_open = false;

                        // Calculate cursor position in text
                        let font = self.font.unwrap_or_else(|| renderer.default_font());
                        let text_size = self.text_size.unwrap_or(Pixels(14.0));

                        let relative_x = cursor_position.x - input_bounds.x - self.padding.left;
                        state.cursor_position = self.cursor_position_from_x(
                            &self.value,
                            relative_x,
                            font,
                            text_size.0,
                            renderer,
                        );

                        return event::Status::Captured;
                    } else {
                        // Click outside - unfocus
                        state.is_focused = false;
                        state.is_open = false;
                    }
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) => {
                if state.is_focused {
                    match key.as_ref() {
                        keyboard::Key::Character(c) => {
                            if !modifiers.control() && !modifiers.alt() {
                                let mut value = self.value.clone();
                                value.insert_str(state.cursor_position, c);
                                state.cursor_position += c.len();
                                shell.publish((self.on_input)(value));
                                return event::Status::Captured;
                            }
                        }
                        keyboard::Key::Named(keyboard::key::Named::Backspace) => {
                            if state.cursor_position > 0 {
                                let value = self.value.clone();
                                let mut chars: Vec<char> = value.chars().collect();
                                if !chars.is_empty() && state.cursor_position <= chars.len() {
                                    chars.remove(state.cursor_position - 1);
                                    state.cursor_position -= 1;
                                    shell.publish((self.on_input)(chars.into_iter().collect()));
                                }
                                return event::Status::Captured;
                            }
                        }
                        keyboard::Key::Named(keyboard::key::Named::Delete) => {
                            let value = self.value.clone();
                            let mut chars: Vec<char> = value.chars().collect();
                            if state.cursor_position < chars.len() {
                                chars.remove(state.cursor_position);
                                shell.publish((self.on_input)(chars.into_iter().collect()));
                            }
                            return event::Status::Captured;
                        }
                        keyboard::Key::Named(keyboard::key::Named::ArrowLeft) => {
                            if state.cursor_position > 0 {
                                state.cursor_position -= 1;
                            }
                            return event::Status::Captured;
                        }
                        keyboard::Key::Named(keyboard::key::Named::ArrowRight) => {
                            if state.cursor_position < self.value.len() {
                                state.cursor_position += 1;
                            }
                            return event::Status::Captured;
                        }
                        keyboard::Key::Named(keyboard::key::Named::Enter) => {
                            if let Some(on_submit) = &self.on_submit {
                                shell.publish(on_submit.clone());
                            }
                            return event::Status::Captured;
                        }
                        keyboard::Key::Named(keyboard::key::Named::Escape) => {
                            state.is_focused = false;
                            state.is_open = false;
                            return event::Status::Captured;
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
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let state = tree.state.downcast_mut::<State<Renderer::Paragraph>>();
        let font = self.font.unwrap_or_else(|| renderer.default_font());

        if state.is_open {
            let bounds = layout.bounds();

            let on_select = &self.on_select;

            let mut menu = Menu::new(
                &mut state.menu,
                self.items.borrow(),
                &mut state.hovered_option,
                |option| {
                    state.is_open = false;

                    (on_select)(option)
                },
                None,
                &self.menu_class,
            )
            .width(bounds.width)
            .padding(self.padding)
            .font(font)
            .text_shaping(self.text_shaping);

            if let Some(text_size) = self.text_size {
                menu = menu.text_size(text_size);
            }

            Some(menu.overlay(layout.position() + translation, bounds.height))
        } else {
            None
        }
    }

    fn size(&self) -> Size<Length> {
        Size::new(self.width, Length::Shrink)
    }

    fn layout(
        &self,
        _tree: &mut widget::Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let text_size = self.text_size.unwrap_or(Pixels(14.0)).0;

        // Basic height calculation based on text size and padding
        let height = text_size + self.padding.vertical();
        let height = height.max(32.0); // Minimum height for button

        let size = limits.resolve(self.width, Length::Fixed(height.into()), Size::ZERO);

        layout::Node::new(size)
    }

    fn draw(
        &self,
        tree: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State<Renderer::Paragraph>>();
        let bounds = layout.bounds();
        let font = self.font.unwrap_or_else(|| renderer.default_font());
        let text_size = self.text_size.unwrap_or(Pixels(14.0));

        // Button bounds (rightmost 32px)
        let button_width = 32.0;
        let button_bounds = Rectangle {
            x: bounds.x + bounds.width - button_width,
            y: bounds.y,
            width: button_width,
            height: bounds.height,
        };

        // Text input bounds
        let input_bounds = Rectangle {
            x: bounds.x,
            y: bounds.y,
            width: bounds.width - button_width,
            height: bounds.height,
        };

        // Draw text input background
        let input_appearance = iced::widget::text_input::Style {
            background: iced::Background::Color(Color::WHITE),
            border: Border {
                color: if state.is_focused {
                    Color::from_rgb(0.0, 0.5, 1.0)
                } else {
                    Color::from_rgb(0.7, 0.7, 0.7)
                },
                width: 1.0,
                radius: iced::border::Radius::new(4.0),
            },
            icon: Color::BLACK,
            placeholder: Color::from_rgb(0.5, 0.5, 0.5),
            value: Color::BLACK,
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
                shaping: self.text_shaping,
                horizontal_alignment: iced::alignment::Horizontal::Left,
                vertical_alignment: iced::alignment::Vertical::Center,
                wrapping: text::Wrapping::None,
            },
            text_bounds.position(),
            text_color,
            text_bounds,
        );

        // Draw cursor if focused
        if state.is_focused {
            let cursor_x =
                self.cursor_x_position(renderer, font, text_size.0, state.cursor_position);
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
                iced::Background::Color(text_color),
            );
        }

        // Draw button
        let button_appearance = iced::widget::button::Style {
            background: Some(iced::Background::Color(Color::from_rgb(0.9, 0.9, 0.9))),
            text_color: Color::BLACK,
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
                border: Border {
                    color: button_appearance.border.color,
                    width: button_appearance.border.width,
                    radius: iced::border::Radius::new(4.0),
                },
                shadow: iced_core::Shadow::default(),
            },
            button_appearance
                .background
                .unwrap_or(iced::Background::Color(Color::TRANSPARENT)),
        );

        // Draw dropdown arrow
        let arrow_bounds = Rectangle {
            x: button_bounds.x + button_bounds.width / 2.0 - 4.0,
            y: button_bounds.y + button_bounds.height / 2.0 - 6.0,
            width: 8.0,
            height: 12.0,
        };

        renderer.fill_text(
            text::Text {
                content: "▼".to_string(),
                size: text_size,
                line_height: text::LineHeight::default(),
                font,
                bounds: arrow_bounds.size(),
                shaping: text::Shaping::default(),
                horizontal_alignment: iced::alignment::Horizontal::Center,
                vertical_alignment: iced::alignment::Vertical::Center,
                wrapping: text::Wrapping::None,
            },
            arrow_bounds.position(),
            button_appearance.text_color,
            arrow_bounds,
        );
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
        let button_width = 32.0;
        let button_bounds = Rectangle {
            x: bounds.x + bounds.width - button_width,
            y: bounds.y,
            width: button_width,
            height: bounds.height,
        };

        if let Some(cursor_position) = cursor.position()
            && (button_bounds.contains(cursor_position) || bounds.contains(cursor_position))
        {
            return mouse::Interaction::Pointer;
        }

        mouse::Interaction::default()
    }
}

#[derive(Debug)]
struct State<P: text::Paragraph> {
    menu: menu::State,
    keyboard_modifiers: keyboard::Modifiers,
    is_open: bool,
    is_focused: bool,
    cursor_position: usize,
    hovered_option: Option<usize>,
    options: Vec<paragraph::Plain<P>>,
    placeholder: paragraph::Plain<P>,
}

impl<P: text::Paragraph> State<P> {
    /// Creates a new [`State`] for a [`TextInputDropdown`].
    fn new() -> Self {
        Self {
            menu: menu::State::default(),
            keyboard_modifiers: keyboard::Modifiers::default(),
            is_open: bool::default(),
            is_focused: bool::default(),
            cursor_position: 0,
            hovered_option: Option::default(),
            options: Vec::new(),
            placeholder: paragraph::Plain::default(),
        }
    }
}

impl<P: text::Paragraph> Default for State<P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T, L, Message, Theme, Renderer> TextInputDropdown<'a, T, L, Message, Theme, Renderer>
where
    T: ToString + PartialEq + Clone,
    L: Borrow<[T]> + 'a,
    Theme: Catalog + iced::widget::text_input::Catalog + iced::widget::button::Catalog,
    Renderer: text::Renderer,
{
    fn cursor_x_position(
        &self,
        _renderer: &Renderer,
        _font: Renderer::Font,
        text_size: f32,
        cursor_position: usize,
    ) -> f32 {
        if self.value.is_empty() {
            return 0.0;
        }

        let chars: Vec<char> = self.value.chars().collect();
        let cursor_pos = std::cmp::min(cursor_position, chars.len());

        let text_before_cursor: String = chars.iter().take(cursor_pos).collect();

        // Simple approximation - in a real implementation you'd want to use proper text measurement
        text_before_cursor.len() as f32 * (text_size * 0.6)
    }

    fn cursor_position_from_x(
        &self,
        value: &str,
        x: f32,
        _font: Renderer::Font,
        text_size: f32,
        _renderer: &Renderer,
    ) -> usize {
        let char_width = text_size * 0.6; // Approximation
        let position = (x / char_width) as usize;
        std::cmp::min(position, value.len())
    }
}

impl<'a, T, L, Message, Theme, Renderer> From<TextInputDropdown<'a, T, L, Message, Theme, Renderer>>
    for iced::Element<'a, Message, Theme, Renderer>
where
    T: Clone + ToString + PartialEq + 'a,
    L: Borrow<[T]>,
    Message: Clone + 'a,
    Theme: Catalog + iced::widget::text_input::Catalog + iced::widget::button::Catalog + 'a,
    Renderer: text::Renderer + 'a,
{
    fn from(widget: TextInputDropdown<'a, T, L, Message, Theme, Renderer>) -> Self {
        iced::Element::new(widget)
    }
}
