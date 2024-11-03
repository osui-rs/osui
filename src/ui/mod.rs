pub mod styles;
pub use styles::*;
pub mod elements;
pub use elements::*;

/// Creates and returns a `Box` containing a new `Text` element.
///
/// This function constructs a `Text` element and wraps it in a `Box` to
/// manage heap allocation and ownership.
///
/// # Returns
///
/// A boxed `Text` element, initialized and ready to be styled or populated
/// with text content.
pub fn text() -> Box<Text> {
    Box::new(Text::new())
}

/// Creates and returns a `Box` containing a new `Button` element.
///
/// This function constructs a `Button` element and wraps it in a `Box` to
/// manage heap allocation and ownership. Buttons can be styled and
/// configured for interactive functionality.
///
/// # Returns
///
/// A boxed `Button` element, initialized and ready for configuration.
pub fn button() -> Box<Button> {
    let mut btn = Button::new();
    btn.style.clicked_bg = styles::Color::White;
    btn.style.clicked_fg = styles::Color::Black;
    Box::new(btn)
}
