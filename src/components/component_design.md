# NetMonkey Component Design Guide

This document outlines the design patterns, theming system, and architectural guidelines for creating components in the NetMonkey network monitoring application.

## 📋 Table of Contents

1. [Project Overview](#project-overview)
2. [Architecture Patterns](#architecture-patterns)
3. [Theming System](#theming-system)
4. [Component Structure](#component-structure)
5. [Design Principles](#design-principles)
6. [Implementation Guidelines](#implementation-guidelines)
7. [Examples](#examples)
8. [Best Practices](#best-practices)

## 🎯 Project Overview

NetMonkey is a network monitoring application built with Iced (Rust GUI framework) that provides:

- **IP scanning and network discovery**
- **TCP/UDP connection testing**
- **Real-time network monitoring**
- **Professional dark-theme interface optimized for monitoring environments**

### Target Users
- Network administrators
- System administrators
- DevOps engineers
- Anyone needing professional network diagnostic tools

## 🏗️ Architecture Patterns

### Component Organization
```
src/components/
├── mod.rs              # Component exports
├── component_design.md # This design guide
├── dropdown.rs         # Generic dropdown with text input
├── selection_overlay.rs # Dropdown overlay implementation
└── subnet_slider.rs    # Custom subnet mask slider
```

### Message-Driven Architecture
All components follow Iced's message-driven pattern:

```rust
#[derive(Debug, Clone)]
pub enum Message {
    // Component-specific messages
    ValueChanged(T),
    ItemSelected(T),
    // State change messages
    FocusChanged(bool),
}
```

### Generic Component Pattern
Components are designed to be reusable with generic type parameters:

```rust
pub struct CustomComponent<Message, Theme = iced::Theme> {
    // Core functionality
    value: T,
    on_change: Box<dyn Fn(T) -> Message>,
    
    // Styling properties
    width: Length,
    height: f32,
    text_size: f32,
    
    // Theme support (when needed)
    _phantom: PhantomData<Theme>,
}
```

## 🎨 Theming System

### Color Palette Philosophy
NetMonkey uses a **professional monitoring-focused** color scheme:

- **Dark background** (reduces eye strain during long monitoring sessions)
- **High contrast text** (ensures readability)
- **Subtle accent colors** (network blue, success green, warning orange, danger red)
- **Hierarchical backgrounds** (main → menu → sub-menu progression)

### Available Themes

#### 1. Dark Theme (Default)
```rust
NetMonkeyColors::DARK = {
    background: Color::from_rgb(0.08, 0.08, 0.12),    // Very dark blue-gray
    menu: Color::from_rgb(0.12, 0.12, 0.18),          // Navigation background
    sub_menu: Color::from_rgb(0.16, 0.16, 0.22),      // Sub-navigation
    text: Color::from_rgb(0.92, 0.92, 0.96),          // Light text
    primary: Color::from_rgb(0.2, 0.6, 1.0),          // Network blue
    success: Color::from_rgb(0.0, 0.8, 0.4),          // Connection success
    warning: Color::from_rgb(1.0, 0.7, 0.0),          // Timeouts/warnings
    danger: Color::from_rgb(1.0, 0.3, 0.3),           // Connection errors
}
```

#### 2. Light Theme
- Inverted color relationships for bright environments
- Maintains same contrast ratios

#### 3. High Contrast Theme
- Maximum contrast for accessibility
- Clear black/white relationships

### Theme Integration Patterns

#### For Canvas Components
```rust
impl<Message> canvas::Program<Message> for MyCanvasComponent<Message> {
    fn draw(&self, theme: &iced::Theme, ...) -> Vec<Geometry> {
        let palette = theme.palette();
        
        // Use theme colors
        frame.fill(&background, palette.background);
        frame.fill_text(text, palette.text);
    }
}
```

#### For Widget Components
```rust
container(content)
    .style(|theme: &Theme| {
        let palette = theme.palette();
        container::Style {
            background: Some(iced::Background::Color(palette.background)),
            border: iced::Border {
                color: palette.primary,
                width: 1.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        }
    })
```

#### Using Theme Helpers
```rust
use crate::theme::helpers;

// Create themed containers
let menu = helpers::menu_container(content, current_theme);
let sub_menu = helpers::sub_menu_container(content, current_theme);

// Create status text
let status = helpers::status_text(
    "Connection successful", 
    helpers::StatusType::Success,
    current_theme
);
```

## 🔧 Component Structure

### Standard Component Template
```rust
use iced::{Element, Length, Color, Theme};
use std::marker::PhantomData;

/// Component description with features and examples
///
/// # Features
/// - Feature 1 description
/// - Feature 2 description
/// 
/// # Examples
/// ```rust
/// let component = MyComponent::new(value, |v| Message::ValueChanged(v))
///     .text_size(16.0)
///     .width(Length::Fill);
/// ```
pub struct MyComponent<Message> {
    // Core state
    value: T,
    on_change: Box<dyn Fn(T) -> Message>,
    
    // Styling properties
    width: Length,
    height: f32,
    text_size: f32,
}

impl<Message> MyComponent<Message> {
    /// Creates a new component instance
    pub fn new<F>(value: T, on_change: F) -> Self
    where
        F: Fn(T) -> Message + 'static,
    {
        Self {
            value,
            on_change: Box::new(on_change),
            width: Length::Fill,
            height: 40.0,
            text_size: 14.0,
        }
    }
    
    /// Chainable styling methods
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }
    
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }
    
    pub fn text_size(mut self, size: f32) -> Self {
        self.text_size = size;
        self
    }
    
    /// Convert to Element
    pub fn into_element(self) -> Element<'static, Message>
    where
        Message: 'static + Clone,
    {
        // Implementation specific to component type
    }
}
```

### Canvas Component Pattern
For custom drawing components (like SubnetSlider):

```rust
struct MyCanvasComponent<Message> {
    value: T,
    on_change: Box<dyn Fn(T) -> Message>,
    text_size: f32,
}

impl<Message> canvas::Program<Message> for MyCanvasComponent<Message>
where
    Message: Clone,
{
    type State = MyComponentState;
    
    fn draw(&self, state: &Self::State, renderer: &Renderer, theme: &iced::Theme, bounds: Rectangle, cursor: mouse::Cursor) -> Vec<Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let palette = theme.palette();
        
        // Draw using theme colors
        frame.fill(&background, palette.background);
        frame.fill_text(Text {
            content: self.value.to_string(),
            color: palette.text,
            size: iced::Pixels(self.text_size),
            // ... other properties
        });
        
        vec![frame.into_geometry()]
    }
    
    fn update(&self, state: &mut Self::State, event: canvas::Event, bounds: Rectangle, cursor: mouse::Cursor) -> (canvas::event::Status, Option<Message>) {
        // Handle interaction events
    }
    
    fn mouse_interaction(&self, state: &Self::State, bounds: Rectangle, cursor: mouse::Cursor) -> mouse::Interaction {
        if cursor.is_over(bounds) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }
}
```

## 📐 Design Principles

### 1. Professional Monitoring Aesthetic
- **Dark theme by default** (reduces eye strain)
- **Subtle borders and shadows**
- **Consistent spacing** (8px, 12px, 16px, 20px grid)
- **Rounded corners** (4px radius for components)

### 2. Functional Design
- **Information density** over visual flair
- **Clear visual hierarchy**
- **Status indication** through color coding
- **Responsive layouts** that work at different window sizes

### 3. Accessibility
- **High contrast text**
- **Keyboard navigation support**
- **Screen reader friendly**
- **Color-blind friendly** status indicators

### 4. Consistency
- **Unified color palette**
- **Consistent component behavior**
- **Standard sizing** (text: 12px, 14px, 16px, 18px, 24px)
- **Predictable interactions**

## 🛠️ Implementation Guidelines

### Component Lifecycle
1. **Define the component struct** with necessary fields
2. **Implement constructor** with sensible defaults
3. **Add chainable styling methods**
4. **Implement core functionality** (Widget trait or Canvas Program)
5. **Add theme support** using palette colors
6. **Write comprehensive documentation**
7. **Add unit tests** for core functionality

### Error Handling
```rust
// Graceful degradation
pub fn new(value: u8, on_change: F) -> Self {
    Self {
        value: value.clamp(1, 32), // Ensure valid range
        // ... other fields
    }
}

// Input validation
fn update_value(&mut self, new_value: String) {
    if let Ok(parsed) = new_value.parse::<u8>() {
        self.value = parsed.clamp(1, 32);
    }
    // Invalid input is ignored, maintaining current state
}
```

### State Management
- **Minimal state** - components should be as stateless as possible
- **External state** - application state should live in the main app
- **Event-driven updates** - use messages for all state changes

### Performance Considerations
- **Lazy rendering** - only redraw when necessary
- **Efficient callbacks** - avoid expensive operations in event handlers
- **Memory efficiency** - prefer references over cloning when possible

## 📚 Examples

### Simple Themed Container
```rust
use crate::theme::{NetMonkeyTheme, helpers};

// Using helper functions
let menu_container = helpers::menu_container(
    column![
        text("Network Tools"),
        button("Port Scanner"),
        button("IP Scanner"),
    ],
    NetMonkeyTheme::Dark
);

// Manual styling
let custom_container = container(content)
    .style(|theme: &Theme| {
        let colors = NetMonkeyTheme::Dark.colors();
        container::Style {
            background: Some(iced::Background::Color(colors.menu)),
            border: iced::Border {
                color: colors.border,
                width: 1.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        }
    })
    .padding(8);
```

### Status Indicators
```rust
// Using theme helpers
let status_text = helpers::status_text(
    "Connection successful",
    helpers::StatusType::Success,
    current_theme
);

// Custom status styling
let status_color = match connection_status {
    ConnectionStatus::Connected => theme_colors.success,
    ConnectionStatus::Connecting => theme_colors.warning,
    ConnectionStatus::Failed => theme_colors.danger,
    ConnectionStatus::Idle => theme_colors.text_secondary,
};

text(status_message).color(status_color)
```

### Interactive Component with State
```rust
#[derive(Debug, Clone)]
pub struct ComponentState {
    is_focused: bool,
    is_dragging: bool,
    hover_position: Option<Point>,
}

impl Default for ComponentState {
    fn default() -> Self {
        Self {
            is_focused: false,
            is_dragging: false,
            hover_position: None,
        }
    }
}

// In the update method
fn update(&self, state: &mut ComponentState, event: Event) -> (Status, Option<Message>) {
    match event {
        Event::Mouse(mouse::Event::CursorEntered) => {
            state.hover_position = Some(cursor_position);
            (Status::Captured, None)
        }
        Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
            state.is_dragging = true;
            let value = self.calculate_value_from_position(cursor_position);
            (Status::Captured, Some((self.on_change)(value)))
        }
        // ... other events
    }
}
```

## ✅ Best Practices

### DO ✅

- **Use theme.palette()** for all colors
- **Provide chainable builder methods** for styling
- **Include comprehensive documentation** with examples
- **Handle edge cases gracefully** (invalid input, extreme values)
- **Follow the 8px spacing grid**
- **Use consistent border radius** (4px for most components)
- **Implement proper focus states** for accessibility
- **Add unit tests** for core functionality
- **Use descriptive variable names** (especially for colors and sizes)

### DON'T ❌

- **Don't hardcode colors** - always use theme system
- **Don't ignore accessibility** - support keyboard navigation
- **Don't overcomplicate** - prefer simple, functional designs
- **Don't break consistency** - follow established patterns
- **Don't forget error handling** - validate inputs and provide fallbacks
- **Don't skip documentation** - other developers need to understand your component
- **Don't ignore performance** - avoid unnecessary redraws
- **Don't make assumptions** - handle edge cases explicitly

### Component Checklist

Before submitting a new component:

- [ ] **Theme Integration**: Uses `theme.palette()` for all colors
- [ ] **Documentation**: Comprehensive docs with examples
- [ ] **Builder Pattern**: Chainable methods for styling
- [ ] **Error Handling**: Graceful handling of invalid inputs
- [ ] **Accessibility**: Keyboard navigation and screen reader support
- [ ] **Consistency**: Follows established design patterns
- [ ] **Tests**: Unit tests for core functionality
- [ ] **Performance**: Efficient rendering and event handling
- [ ] **Responsive**: Works at different window sizes
- [ ] **Professional**: Matches monitoring application aesthetic

## 🔄 Evolution and Maintenance

### Adding New Features
1. **Assess impact** on existing components
2. **Update theme system** if new colors are needed
3. **Maintain backward compatibility** when possible
4. **Update documentation** and examples
5. **Test across all themes** (Dark, Light, High Contrast)

### Theme Evolution
- **Extend NetMonkeyColors** for new color needs
- **Update all existing components** to use new colors
- **Maintain contrast ratios** for accessibility
- **Test with actual network monitoring workflows**

### Component Versioning
- **Major changes**: Breaking API changes
- **Minor changes**: New features, backward compatible
- **Patch changes**: Bug fixes, performance improvements

---

*This guide is a living document. Update it as the project evolves and new patterns emerge.*