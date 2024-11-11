/// The `ui` module provides user interface components and utilities for building
/// a text-based user interface (TUI). It includes predefined styles and elements
/// such as text, buttons, and containers.

pub mod styles;
pub use styles::*;
pub mod elements;
pub use elements::*;

pub fn text<'a>() -> Box<Text<'a>> {
    Box::new(Text::new())
}