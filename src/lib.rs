//! # Library Documentation
//! This library provides a framework for creating terminal-based user interfaces.
//! It includes support for widgets, layouts, and event handling using `crossterm`.

//! ## Modules
//! - `console`: Handles terminal input and events.
//! - `elements`: Provides pre-defined UI elements. Optional, enabled by default.
//! - `rsx`: Supports declarative UI definition using an XML-like syntax. Optional, enabled by default.
//! - `state`: Manages state for widgets and other components.
//! - `utils`: Utility functions for terminal rendering.
//! - `prelude`: Exports commonly used types and traits for easy access.

pub mod console;
#[cfg(not(feature = "no_elem"))]
pub mod elements;
#[cfg(not(feature = "no_rsx"))]
pub mod rsx;
pub mod state;
pub mod utils;

/// Commonly used imports for convenience.
pub mod prelude {
    pub use crate::elements::*;
    pub use crate::*;
    pub use console::Event;
    pub use crossterm::event::{KeyCode, KeyEvent};
}

/// Type alias for simplifying error handling.
/// Represents an I/O operation result.
pub use std::io::Result;

/// Type alias for a UI element.
/// An `Element` is a thread-safe function that renders onto a `Frame`
/// and optionally handles events.
pub type Element = std::sync::Arc<dyn Fn(&mut Frame, Option<console::Event>) -> crate::Result<()>>;

/// A trait representing a UI widget.
pub trait Widget {
    /// Renders the widget as a `String`.
    fn render(&self) -> String;

    /// Handles an event for the widget. Defaults to a no-op.
    fn event(&mut self, event: console::Event) {
        _ = event;
    }
}

/// Struct representing a rectangular area in the terminal.
#[derive(Debug, Clone, Copy)]
pub struct Area {
    pub width: Size,
    pub height: Size,
    pub x: Pos,
    pub y: Pos,
}

/// Struct representing a rendering frame.
#[derive(Debug, Default, Clone, Copy)]
pub struct Frame {
    pub width: u16,
    pub height: u16,
    last_elem: (u16, u16),
}

/// Enum representing positioning options for widgets.
#[derive(Debug, Clone, Copy)]
pub enum Pos {
    /// Automatic positioning.
    #[allow(non_camel_case_types)]
    auto,
    /// Centered positioning.
    #[allow(non_camel_case_types)]
    center,
    /// Fixed position as a number of columns or rows.
    Num(u16),
}

/// Enum representing sizing options for widgets.
#[derive(Debug, Clone, Copy)]
pub enum Size {
    /// Automatic sizing.
    Auto,
    /// Fixed size.
    Num(u16),
}

impl Pos {
    /// Calculates the position based on the current frame dimensions.
    pub fn get(self, auto: u16, width: u16, frame: u16) -> u16 {
        match self {
            Self::auto => auto,
            Self::center => (frame - width) / 2,
            Self::Num(n) => n,
        }
    }
}

impl Size {
    /// Internal method to compute size.
    fn get_(self, written: u16) -> u16 {
        match self {
            Self::Auto => written,
            Self::Num(n) => n,
        }
    }

    /// Computes the size based on written content or frame dimensions.
    pub fn get(self, written: u16, _frame: u16) -> u16 {
        self.get_(written)
    }
}

impl Area {
    /// Creates a new `Area` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Center the `Area` horizontally. Only available with the `portable` feature.
    #[cfg(feature = "portable")]
    pub fn center_x() -> Self {
        let mut s = Self::default();
        s.x = Pos::center;
        s
    }

    /// Center the `Area` vertically. Only available with the `portable` feature.
    #[cfg(feature = "portable")]
    pub fn center_y() -> Self {
        let mut s = Self::default();
        s.y = Pos::center;
        s
    }

    /// Center the `Area` both horizontally and vertically. Only available with the `portable` feature.
    #[cfg(feature = "portable")]
    pub fn center() -> Self {
        let mut s = Self::default();
        s.x = Pos::center;
        s.y = Pos::center;
        s
    }

    // Additional utility methods for setting properties are available with the `portable` feature.
    #[cfg(feature = "portable")]
    pub fn set_width(&mut self, w: Size) -> Self {
        self.width = w;
        *self
    }
    #[cfg(feature = "portable")]
    pub fn set_height(&mut self, h: Size) -> Self {
        self.height = h;
        *self
    }
    #[cfg(feature = "portable")]
    pub fn set_x(&mut self, x: Pos) -> Self {
        self.x = x;
        *self
    }
    #[cfg(feature = "portable")]
    pub fn set_y(&mut self, y: Pos) -> Self {
        self.y = y;
        *self
    }
}

impl Default for Area {
    fn default() -> Self {
        Self {
            width: Size::Auto,
            height: Size::Auto,
            x: Pos::Num(0),
            y: Pos::auto,
        }
    }
}

impl Frame {
    /// Draws a widget on the frame.
    pub fn draw<W>(&mut self, w: &W, props: Area) -> Result<()>
    where
        W: Widget,
    {
        let written = w.render();

        let (ww, wh) = utils::str_size(&written);

        let (width, height) = (
            props.width.get(ww, self.width),
            props.height.get(wh, self.height),
        );

        let (x, y) = (
            props.x.get(self.last_elem.0, width, self.width),
            props.y.get(self.last_elem.1, height, self.height),
        );

        for (i, line) in written.lines().enumerate() {
            if i as u16 > height {
                break;
            }

            println!(
                "\x1b[{};{}H{}",
                y + (i as u16) + 1,
                x + 1,
                line.chars().take(width as usize).collect::<String>()
            );
        }

        self.last_elem.0 = x + width;
        self.last_elem.1 = y + height;

        Ok(())
    }

    /// Clears the frame.
    pub fn clear(&mut self) -> Result<()> {
        self.last_elem.0 = 0;
        self.last_elem.1 = 0;
        utils::clear()
    }

    /// Creates a new frame with the specified dimensions.
    pub fn new((width, height): (u16, u16)) -> Self {
        Self {
            width,
            height,
            last_elem: (0, 0),
        }
    }
}

/// Creates a new state object.
pub fn use_state<T>(v: T) -> state::State<T> {
    state::State(Box::into_raw(Box::new(v)))
}
