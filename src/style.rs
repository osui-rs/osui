//! Layout and style definitions for OSUI widgets.
//!
//! This module defines the structures used to manage rendering geometry (`Transform`, `RawTransform`),
//! position and size enums (`Position`, `Dimension`), and styling (`Style`, `Background`, etc.).
//!
//! These types are used internally by components to calculate layout and control their appearance.

use crate::component;

/// Resolved layout information for a widget after layout calculations.
///
/// This struct holds concrete values for position (`x`, `y`), dimensions
/// (`width`, `height`), and padding (`px`, `py`).
#[derive(Debug, Clone)]
pub struct RawTransform {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub px: u16,
    pub py: u16,
}

/// Horizontal or vertical position relative to a parent container.
#[derive(Debug, Clone)]
pub enum Position {
    /// Fixed position in cells from the origin.
    Const(u16),
    /// Centered in the parent.
    Center,
    /// Aligned to the end (right or bottom) of the parent.
    End,
}

/// Sizing rule for width or height.
#[derive(Debug, Clone)]
pub enum Dimension {
    /// Fills the available space from the parent.
    Full,
    /// Automatically sized to fit content.
    Content,
    /// Fixed size in cells.
    Const(u16),
}

/// Background appearance for a widget.
#[derive(Debug, Clone)]
pub enum Background {
    /// Transparent / no background.
    NoBackground,
    /// Draws a basic outline using the given color.
    Outline(u32),
    /// Draws a rounded outline using the given color.
    RoundedOutline(u32),
    /// Fills the background with the specified color.
    Solid(u32),
}

component!(Transform {
    pub x: Position,
    pub y: Position,
    pub mx: i32,
    pub my: i32,
    pub px: u16,
    pub py: u16,
    pub width: Dimension,
    pub height: Dimension,
});

component!(Style {
    pub background: Background,
    pub foreground: Option<u32>,
});

impl Style {
    /// Creates a style with no background or foreground.
    pub fn new() -> Self {
        Self {
            background: Background::NoBackground,
            foreground: None,
        }
    }
}

impl Transform {
    /// Creates a default transform with top-left alignment and content sizing.
    pub fn new() -> Transform {
        Transform {
            x: Position::Const(0),
            y: Position::Const(0),
            mx: 0,
            my: 0,
            px: 0,
            py: 0,
            width: Dimension::Content,
            height: Dimension::Content,
        }
    }

    /// Shortcut for centering both horizontally and vertically.
    pub fn center() -> Transform {
        Transform {
            x: Position::Center,
            y: Position::Center,
            mx: 0,
            my: 0,
            px: 0,
            py: 0,
            width: Dimension::Content,
            height: Dimension::Content,
        }
    }

    /// Aligns the widget to the bottom of its parent.
    pub fn bottom(mut self) -> Self {
        self.y = Position::End;
        self
    }

    /// Aligns the widget to the right of its parent.
    pub fn right(mut self) -> Self {
        self.x = Position::End;
        self
    }

    /// Adds margin (offset) from parent edge.
    pub fn margin(mut self, x: i32, y: i32) -> Self {
        self.mx = x;
        self.my = y;
        self
    }

    /// Adds internal spacing (padding) around content.
    pub fn padding(mut self, x: u16, y: u16) -> Self {
        self.px = x;
        self.py = y;
        self
    }

    /// Sets constant dimensions.
    pub fn dimensions(mut self, width: u16, height: u16) -> Self {
        self.width = Dimension::Const(width);
        self.height = Dimension::Const(height);
        self
    }

    /// Resolves `Dimension` rules into absolute values for the given parent size.
    pub fn use_dimensions(&self, parent_width: u16, parent_height: u16, raw: &mut RawTransform) {
        self.width.use_dimension(parent_width, &mut raw.width);
        self.height.use_dimension(parent_height, &mut raw.height);
    }

    /// Resolves `Position` rules into absolute positions for the given parent size.
    pub fn use_position(&self, parent_width: u16, parent_height: u16, raw: &mut RawTransform) {
        self.x
            .use_position(raw.width, parent_width, self.mx, &mut raw.x);
        self.y
            .use_position(raw.height, parent_height, self.my, &mut raw.y);
    }
}

impl RawTransform {
    /// Creates a new transform with all fields set to 0.
    pub fn new() -> RawTransform {
        RawTransform {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            px: 0,
            py: 0,
        }
    }
}

impl Dimension {
    /// Applies a dimension rule to determine a final size.
    pub fn use_dimension(&self, parent: u16, r: &mut u16) {
        match self {
            Self::Full => *r = parent,
            Self::Content => {}
            Self::Const(n) => *r = *n,
        }
    }
}

impl Position {
    /// Applies a position rule to determine a final coordinate, based on size and parent.
    pub fn use_position(&self, size: u16, parent: u16, m: i32, r: &mut u16) {
        let base = match self {
            Self::Center => {
                if parent < size {
                    return;
                }
                (parent - size) / 2
            }
            Self::Const(n) => *n,
            Self::End => {
                if parent < size {
                    return;
                }
                parent - size
            }
        };

        let adjusted = if m >= 0 {
            base.checked_add(m as u16)
        } else {
            base.checked_sub((-m) as u16)
        };

        if let Some(val) = adjusted {
            *r = val;
        }
    }
}

impl From<u16> for Position {
    fn from(value: u16) -> Self {
        Self::Const(value)
    }
}

impl From<u16> for Dimension {
    fn from(value: u16) -> Self {
        Self::Const(value)
    }
}
