use iced::mouse;
use iced::widget::canvas::{self, Canvas, Geometry, Path, Stroke, Text};
use iced::{Color, Element, Length, Point, Rectangle, Renderer, Size};

/// A custom subnet slider that looks like a progress bar with text overlay.
/// The left side shows dotted decimal notation, right side shows CIDR notation,
/// and the slider appears as a draggable blue filled area.
///
/// Features:
/// - Rounded corners and themed styling matching other components
/// - Single white pixel outline for enhanced visibility
/// - Customizable text size for both notations
/// - Draggable interaction with visual feedback
/// - Automatic subnet mask calculations
///
/// # Examples
///
/// ## Basic Usage
/// ```rust
/// use net_monkey::components::subnet_slider::subnet_slider;
///
/// let slider = subnet_slider(24, |value| Message::SubnetChanged(value));
/// let element = slider.into_element();
/// ```
///
/// ## Customized Usage
/// ```rust
/// let slider = subnet_slider(16, |value| Message::SubnetChanged(value))
///     .text_size(18.0)    // Larger text
///     .height(50.0)       // Taller slider
///     .width(Length::Fixed(400.0));
/// ```
///
/// ## Visual Layout
/// ```
/// ⬜[255.255.255.0######         24]⬜
///    ↑              ↑         ↑
/// Left side      Draggable   Right side
/// (calculated)    Slider    (notation)
/// White outline around entire component
/// ```
pub struct SubnetSlider<Message> {
    value: u8,
    on_change: Box<dyn Fn(u8) -> Message>,
    width: Length,
    height: f32,
    text_size: f32,
}

impl<Message> SubnetSlider<Message> {
    /// Creates a new SubnetSlider
    pub fn new<F>(value: u8, on_change: F) -> Self
    where
        F: Fn(u8) -> Message + 'static,
    {
        Self {
            value: value.clamp(1, 32),
            on_change: Box::new(on_change),
            width: Length::Fill,
            height: 40.0,
            text_size: 14.0,
        }
    }

    /// Sets the width of the slider
    #[allow(dead_code)]
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the slider
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Sets the text size for the notation displays
    pub fn text_size(mut self, size: f32) -> Self {
        self.text_size = size;
        self
    }

    /// Converts CIDR notation to dotted decimal notation
    fn to_dotted_decimal(cidr: u8) -> String {
        let cidr = cidr.clamp(1, 32);
        let mask = if cidr == 32 {
            0xFFFFFFFFu32
        } else {
            0xFFFFFFFFu32 << (32 - cidr)
        };

        format!(
            "{}.{}.{}.{}",
            (mask >> 24) & 0xFF,
            (mask >> 16) & 0xFF,
            (mask >> 8) & 0xFF,
            mask & 0xFF
        )
    }

    /// Calculates the fill percentage based on the current value
    #[allow(dead_code)]
    fn fill_percentage(&self) -> f32 {
        (self.value as f32 - 1.0) / 31.0
    }

    /// Convert to Element using Canvas
    pub fn into_element(self) -> Element<'static, Message>
    where
        Message: 'static + Clone,
    {
        Canvas::new(SubnetSliderCanvas {
            value: self.value,
            on_change: self.on_change,
            text_size: self.text_size,
        })
        .width(self.width)
        .height(self.height)
        .into()
    }
}

struct SubnetSliderCanvas<Message> {
    value: u8,
    on_change: Box<dyn Fn(u8) -> Message>,
    text_size: f32,
}

#[derive(Debug, Clone, Default)]
pub struct SubnetSliderState {
    is_dragging: bool,
}

impl<Message> canvas::Program<Message> for SubnetSliderCanvas<Message>
where
    Message: Clone,
{
    type State = SubnetSliderState;

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        let dotted_decimal = SubnetSlider::<Message>::to_dotted_decimal(self.value);
        let cidr_notation = format!("{}", self.value);
        let fill_percentage = (self.value as f32 - 1.0) / 31.0;

        // Draw white outline background (1 pixel larger)
        let corner_radius = 4.0;
        let outline_background = Path::new(|builder| {
            builder.rounded_rectangle(Point::ORIGIN, bounds.size(), corner_radius.into());
        });
        frame.fill(&outline_background, Color::WHITE);

        // Draw main background (inset by 1 pixel)
        let inset_size = Size::new(bounds.width - 2.0, bounds.height - 2.0);
        let main_background = Path::new(|builder| {
            builder.rounded_rectangle(
                Point::new(1.0, 1.0),
                inset_size,
                (corner_radius - 1.0_f32).max(0.0).into(),
            );
        });
        frame.fill(&main_background, theme.palette().background);

        // Draw border using theme colors - use a muted border color
        let border_color = Color {
            r: theme.palette().text.r * 0.4,
            g: theme.palette().text.g * 0.4,
            b: theme.palette().text.b * 0.4,
            a: theme.palette().text.a * 0.8,
        };
        frame.stroke(
            &main_background,
            Stroke::default().with_color(border_color).with_width(1.0),
        );

        // Draw blue filled portion (progress) with rounded corners (inset by 1 pixel)
        let fill_width = (bounds.width - 2.0) * fill_percentage;
        if fill_width > 1.0 {
            let corner_radius = 4.0;
            let fill_rect = Path::new(|builder| {
                builder.rounded_rectangle(
                    Point::new(1.0, 1.0),
                    Size::new(fill_width, bounds.height - 2.0),
                    (corner_radius - 1.0_f32).max(0.0).into(),
                );
            });
            frame.fill(&fill_rect, theme.palette().primary);
        }

        // Draw left text (dotted decimal) - adjust for white outline
        frame.fill_text(Text {
            content: dotted_decimal,
            position: Point::new(9.0, bounds.height / 2.0),
            color: theme.palette().text,
            size: iced::Pixels(self.text_size),
            font: Default::default(),
            horizontal_alignment: iced::alignment::Horizontal::Left,
            vertical_alignment: iced::alignment::Vertical::Center,
            line_height: Default::default(),
            shaping: Default::default(),
        });

        // Draw right text (CIDR notation) - adjust for white outline
        frame.fill_text(Text {
            content: cidr_notation,
            position: Point::new(bounds.width - 9.0, bounds.height / 2.0),
            color: theme.palette().text,
            size: iced::Pixels(self.text_size),
            font: Default::default(),
            horizontal_alignment: iced::alignment::Horizontal::Right,
            vertical_alignment: iced::alignment::Vertical::Center,
            line_height: Default::default(),
            shaping: Default::default(),
        });

        vec![frame.into_geometry()]
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (canvas::event::Status, Option<Message>) {
        match event {
            canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if let Some(cursor_position) = cursor.position_in(bounds) {
                    state.is_dragging = true;
                    let relative_x = cursor_position.x / bounds.width;
                    let new_value = (1.0_f32 + relative_x.clamp(0.0, 1.0) * 31.0).round() as u8;
                    let new_value = new_value.clamp(1, 32);

                    return (
                        canvas::event::Status::Captured,
                        Some((self.on_change)(new_value)),
                    );
                }
            }
            canvas::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                state.is_dragging = false;
                return (canvas::event::Status::Captured, None);
            }
            canvas::Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if state.is_dragging
                    && let Some(cursor_position) = cursor.position_in(bounds)
                {
                    let relative_x = cursor_position.x / bounds.width;
                    let new_value = (1.0_f32 + relative_x.clamp(0.0, 1.0) * 31.0).round() as u8;
                    let new_value = new_value.clamp(1, 32);

                    return (
                        canvas::event::Status::Captured,
                        Some((self.on_change)(new_value)),
                    );
                }
            }
            _ => {}
        }

        (canvas::event::Status::Ignored, None)
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if cursor.is_over(bounds) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dotted_decimal_conversion() {
        assert_eq!(SubnetSlider::<()>::to_dotted_decimal(24), "255.255.255.0");
        assert_eq!(SubnetSlider::<()>::to_dotted_decimal(16), "255.255.0.0");
        assert_eq!(SubnetSlider::<()>::to_dotted_decimal(8), "255.0.0.0");
        assert_eq!(SubnetSlider::<()>::to_dotted_decimal(32), "255.255.255.255");
    }

    #[test]
    fn test_fill_percentage() {
        let slider = SubnetSlider::<()>::new(1, |_| ());
        assert_eq!(slider.fill_percentage(), 0.0);

        let slider = SubnetSlider::<()>::new(16, |_| ());
        assert!((slider.fill_percentage() - 0.483871).abs() < 0.001);

        let slider = SubnetSlider::<()>::new(24, |_| ());
        assert!((slider.fill_percentage() - 0.741935).abs() < 0.001);

        let slider = SubnetSlider::<()>::new(32, |_| ());
        assert_eq!(slider.fill_percentage(), 1.0);
    }
}
