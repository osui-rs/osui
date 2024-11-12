/// The `ui` module provides user interface components and utilities for building
/// a text-based user interface (TUI). It includes predefined styles and elements
/// such as text, buttons, and containers.

pub mod styles;
pub use styles::*;
pub mod elements;
pub use elements::*;

pub fn text<'a>() -> std::sync::Arc<Text<'a>> {
    std::sync::Arc::new(Text::new())
}

pub fn div<'a>() -> std::sync::Arc<Div<'a>> {
    std::sync::Arc::new(Div::new())
}