//! Net Monkey UI Components Library
//!
//! This crate provides reusable UI components for the Net Monkey application,
//! built with the Iced GUI framework.

pub mod dropdown;
pub mod label_with_hint;
pub mod selection_overlay;
pub mod subnet_slider;
pub mod text_input_with_hint;

// Re-export commonly used components for convenience
pub use dropdown::TextInputDropdown;
pub use label_with_hint::{LabelWithHint, label_with_hint, themed_label_with_hint};
pub use selection_overlay::*;
pub use subnet_slider::SubnetSlider;
pub use text_input_with_hint::{
    TextInputWithHint, text_input_with_hint, themed_text_input_with_hint,
};

// Re-export Iced types that components commonly use
pub use iced::{Element, Theme};
pub use iced_widget as widget;
