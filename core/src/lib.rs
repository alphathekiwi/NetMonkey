//! Net Monkey Core Library
//!
//! This crate provides the core networking functionality for the Net Monkey application,
//! including network adapter discovery, IP scanning, and related utilities.

pub mod adaptor;
pub mod scanner;
pub mod tasks;

// Re-export commonly used types for convenience
pub use adaptor::{NetworkAdapter, get_network_adapters};
pub use tasks::{Task, TaskMessage, TaskState};

// Re-export scanner functionality
pub use scanner::*;
