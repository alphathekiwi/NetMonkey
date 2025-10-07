use iced::Pixels;
use iced::widget::pick_list::Catalog;
use iced_core::clipboard::Clipboard;
use iced_core::event::{self, Event};
use iced_core::layout::{self, Layout};
use iced_core::mouse;
use iced_core::overlay;
use iced_core::renderer;
use iced_core::text::{self};
use iced_core::{Border, Color, Padding, Rectangle, Size, Vector};

pub struct DropdownOverlay<'a, T, Message, Theme, Renderer>
where
    T: ToString + PartialEq + Clone,
    Theme: Catalog + iced::widget::text_input::Catalog + iced::widget::button::Catalog,
    Renderer: text::Renderer,
{
    pub items: Vec<T>,
    pub on_select: &'a dyn Fn(T) -> Message,
    pub bounds: Rectangle,
    pub item_height: f32,
    pub text_size: Pixels,
    pub padding: Padding,
    pub text_line_height: text::LineHeight,
    pub text_shaping: text::Shaping,
    pub font: Option<Renderer::Font>,
    pub class: <Theme as Catalog>::Class<'static>,
    pub hovered_option: Option<usize>,
}

impl<'a, T, Message, Theme, Renderer> overlay::Overlay<Message, Theme, Renderer>
    for DropdownOverlay<'a, T, Message, Theme, Renderer>
where
    T: ToString + PartialEq + Clone,
    Theme: Catalog + iced::widget::text_input::Catalog + iced::widget::button::Catalog,
    Renderer: text::Renderer,
    Message: Clone,
{
    fn layout(&mut self, _renderer: &Renderer, _bounds: Size) -> layout::Node {
        layout::Node::new(Size::new(self.bounds.width, self.bounds.height))
            .move_to(iced_core::Point::new(self.bounds.x, self.bounds.y))
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
    ) {
        let bounds = layout.bounds();
        let font = self.font.unwrap_or_else(|| renderer.default_font());

        let pick_list_style =
            <Theme as Catalog>::style(theme, &self.class, iced::widget::pick_list::Status::Active);

        let input_appearance = iced::widget::text_input::Style {
            background: pick_list_style.background,
            border: Border {
                color: Color::from_rgb(0.7, 0.7, 0.7),
                width: 1.0,
                radius: iced::border::Radius::new(4.0),
            },
            icon: pick_list_style.text_color,
            value: pick_list_style.text_color,
            placeholder: Color::from_rgb(0.5, 0.5, 0.5),
            selection: Color::from_rgb(0.0, 0.5, 1.0),
        };

        // Draw dropdown background with shadow
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: Border {
                    color: Color::from_rgb(0.7, 0.7, 0.7),
                    width: 1.0,
                    radius: iced::border::Radius::new(4.0),
                },
                shadow: iced_core::Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                    offset: Vector::new(0.0, 2.0),
                    blur_radius: 4.0,
                },
            },
            input_appearance.background,
        );

        // Draw dropdown items
        for (index, item) in self.items.iter().enumerate() {
            let item_y = bounds.y + (index as f32 * self.item_height);
            let item_bounds = Rectangle {
                x: bounds.x,
                y: item_y,
                width: bounds.width,
                height: self.item_height,
            };

            // Check if we're past the visible area
            if item_y + self.item_height > bounds.y + bounds.height {
                break;
            }

            // Highlight hovered item
            let is_hovered = self.hovered_option == Some(index);
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

            let text_color = match is_hovered {
                true => Color::BLACK,
                false => input_appearance.value,
            };

            // Draw item text
            renderer.fill_text(
                text::Text {
                    content: item.to_string(),
                    size: self.text_size,
                    line_height: self.text_line_height,
                    font,
                    bounds: item_bounds.size(),
                    horizontal_alignment: iced::alignment::Horizontal::Left,
                    vertical_alignment: iced::alignment::Vertical::Center,
                    shaping: self.text_shaping,
                    wrapping: text::Wrapping::None,
                },
                iced_core::Point::new(item_bounds.x + self.padding.left, item_bounds.y)
                    + Vector {
                        x: 0.0,
                        y: item_bounds.height / 2.0,
                    },
                text_color,
                item_bounds,
            );
        }
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut iced_core::Shell<'_, Message>,
    ) -> event::Status {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                let bounds = layout.bounds();
                if let Some(cursor_position) = cursor.position()
                    && bounds.contains(cursor_position)
                {
                    let clicked_index =
                        ((cursor_position.y - bounds.y) / self.item_height) as usize;

                    if clicked_index < self.items.len() {
                        let selected_item = self.items[clicked_index].clone();
                        shell.publish((self.on_select)(selected_item));
                        return event::Status::Captured;
                    }
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                let bounds = layout.bounds();
                if let Some(cursor_position) = cursor.position() {
                    if bounds.contains(cursor_position) {
                        let hovered_index =
                            ((cursor_position.y - bounds.y) / self.item_height) as usize;

                        if hovered_index < self.items.len() {
                            self.hovered_option = Some(hovered_index);
                            return event::Status::Captured;
                        }
                    }
                } else {
                    self.hovered_option = None;
                }
            }
            _ => {}
        }

        event::Status::Ignored
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }
}
