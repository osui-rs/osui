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

/// Creates and returns a `Box` containing a new `Div` element.
///
/// This function constructs a `Div` element and wraps it in a `Box` to
/// manage heap allocation and ownership. A `Div` can be used as a container
/// for other elements or to organize layout.
///
/// # Returns
///
/// A boxed `Div` element, initialized and ready for layout management.
pub fn div() -> Box<Div> {
    Box::new(Div::new())
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
    Box::new(Button::new())
}

/// Creates and returns a `Box` containing a new `Tab` element with custom styling.
///
/// This function constructs a `Tab` element, sets the foreground color
/// of the selected tab to red, and wraps the element in a `Box` to manage
/// heap allocation and ownership.
///
/// # Returns
///
/// A boxed `Tab` element, initialized and styled with a red foreground
/// color for the selected tab.
pub fn tab() -> Box<Tab> {
    let mut elem: Tab = Tab::new();
    elem.style.selected_fg = Color::Red;
    Box::new(elem)
}

/// Creates and returns a `Box` containing a new `Menu` element with custom styling.
///
/// This function constructs a `Menu` element, sets the background and
/// foreground colors of the selected menu item, and defines the cursor
/// color. The element is wrapped in a `Box` for heap allocation and
/// ownership management.
///
/// # Returns
///
/// A boxed `Menu` element, initialized and styled with a magenta background
/// and black foreground for selected items, and a magenta cursor color.
pub fn menu() -> Box<Menu> {
    let mut elem = Menu::new();
    elem.style.selected_bg = Color::Magenta;
    elem.style.selected_fg = Color::Black;
    elem.style.cursor_fg = Color::Magenta;
    Box::new(elem)
}
