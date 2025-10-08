//! Net Monkey Theme Library
//!
//! This crate provides theming functionality for the Net Monkey application,
//! including custom color palettes and theme variants optimized for network
//! monitoring interfaces.

pub mod colors;

// Re-export the main theme types for convenience
pub use colors::{
    NetMonkeyColors, NetMonkeyTheme, SerializableColor, ThemeDefinition, ThemeManager, helpers,
};

// Re-export commonly used Iced types that themes work with
pub use iced::theme::{Palette, palette::Extended};
pub use iced::{Color, Theme};
