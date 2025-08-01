//! Rendering logic and layout scope management for OSUI.
//!
//! `RenderScope` is responsible for handling transformations, parent-child dimensions,
//! stacking draw operations, and rendering styled output to the terminal.
//!
//! This module allows widgets to accumulate rendering commands (text, shapes, colors),
//! and then flush them to the screen with correct styling and positioning.
//!
//! Used internally by OSUI's layout and rendering system.

use std::fmt::Debug;

use crate::{
    style::{RawTransform, Style, Transform},
    utils::{self, hex_ansi_bg},
};

/// Represents a single render instruction.
#[derive(Clone)]
enum RenderMethod {
    /// Plain text rendering at current transform.
    Text(String),
    /// Text rendered with a specific 24-bit color.
    TextColored(String, u32),
    /// A filled rectangle of a given size and background color.
    Rectangle(u16, u16, u32),
}

/// Stores renderable state and transformation data for a single UI widget.
///
/// `RenderScope` tracks dimensions, styles, and draw commands such as text or
/// background rectangles. It accumulates instructions that are later executed in the `draw` method.
#[derive(Clone)]
pub struct RenderScope {
    transform: RawTransform,
    render_stack: Vec<RenderMethod>,
    parent_width: u16,
    parent_height: u16,
    style: Style,
}

impl RenderScope {
    /// Creates a new, empty `RenderScope`.
    pub fn new() -> RenderScope {
        RenderScope {
            transform: RawTransform::new(),
            render_stack: Vec::new(),
            parent_width: 0,
            parent_height: 0,
            style: Style::new(),
        }
    }

    /// Directly sets the raw transform (position and size) for this scope.
    pub fn set_transform_raw(&mut self, transform: RawTransform) {
        self.transform = transform;
    }

    /// Applies a `Transform` to this scope, factoring in parent dimensions.
    pub fn set_transform(&mut self, transform: &Transform) {
        transform.use_dimensions(self.parent_width, self.parent_height, &mut self.transform);
        transform.use_position(self.parent_width, self.parent_height, &mut self.transform);
        self.transform.px = transform.px;
        self.transform.py = transform.py;
    }

    /// Adds a text draw instruction.
    pub fn draw_text(&mut self, text: &str) {
        self.render_stack.push(RenderMethod::Text(text.to_string()));
        let (w, h) = utils::str_size(text);
        self.transform.width = self.transform.width.max(w);
        self.transform.height = self.transform.height.max(h);
    }

    /// Adds a colored text draw instruction.
    pub fn draw_text_colored(&mut self, text: &str, color: u32) {
        self.render_stack
            .push(RenderMethod::TextColored(text.to_string(), color));
        let (w, h) = utils::str_size(text);
        self.transform.width = self.transform.width.max(w);
        self.transform.height = self.transform.height.max(h);
    }

    /// Adds a background rectangle draw instruction.
    pub fn draw_rect(&mut self, width: u16, height: u16, color: u32) {
        self.render_stack
            .push(RenderMethod::Rectangle(width, height, color));
        self.transform.width = self.transform.width.max(width);
        self.transform.height = self.transform.height.max(height);
    }

    /// Manually ensures a minimum area is allocated.
    pub fn use_area(&mut self, width: u16, height: u16) {
        self.transform.width = self.transform.width.max(width);
        self.transform.height = self.transform.height.max(height);
    }

    /// Renders the current stack to the terminal.
    ///
    /// This will draw background styles (e.g. solid fill or outline) first,
    /// followed by each draw instruction in the stack.
    pub fn draw(&self) {
        let width = self.transform.width;
        let height = self.transform.height;

        match self.style.background {
            crate::style::Background::NoBackground => {}
            crate::style::Background::Outline(c) => {
                let width = width + self.transform.px * 2;
                let height = height + self.transform.py * 2;
                if width > 1 && height > 1 {
                    let d = "─".repeat(width as usize - 2);
                    utils::print_liner(
                        self.transform.x,
                        self.transform.y,
                        &utils::hex_ansi(c),
                        &format!(
                            "┌{d}┐{}\n└{d}┘",
                            format!("\n│{}│", " ".repeat(width as usize - 2))
                                .repeat(height as usize - 2),
                        ),
                    );
                }
            }
            crate::style::Background::RoundedOutline(c) => {
                let width = width + self.transform.px * 2;
                let height = height + self.transform.py * 2;
                if width > 1 && height > 1 {
                    let d = "─".repeat(width as usize - 2);
                    utils::print_liner(
                        self.transform.x,
                        self.transform.y,
                        &utils::hex_ansi(c),
                        &format!(
                            "╭{d}╮{}\n╰{d}╯",
                            format!("\n│{}│", " ".repeat(width as usize - 2))
                                .repeat(height as usize - 2),
                        ),
                    );
                }
            }
            crate::style::Background::Solid(c) => utils::print_liner(
                self.transform.x,
                self.transform.y,
                &hex_ansi_bg(c),
                &std::iter::repeat(" ".repeat(width as usize))
                    .take(height as usize)
                    .collect::<Vec<_>>()
                    .join("\n"),
            ),
        }

        for m in &self.render_stack {
            match m {
                RenderMethod::Text(t) => {
                    utils::print(
                        self.transform.x + self.transform.px,
                        self.transform.y + self.transform.py,
                        t,
                    );
                }
                RenderMethod::TextColored(t, c) => utils::print_liner(
                    self.transform.x + self.transform.px,
                    self.transform.y + self.transform.py,
                    &utils::hex_ansi(*c),
                    t,
                ),
                RenderMethod::Rectangle(width, height, color) => {
                    utils::print_liner(
                        self.transform.x + self.transform.px,
                        self.transform.y + self.transform.py,
                        &hex_ansi_bg(*color),
                        &std::iter::repeat(" ".repeat(*width as usize))
                            .take(*height as usize)
                            .collect::<Vec<_>>()
                            .join("\n"),
                    );
                }
            }
        }
    }

    /// Clears all render instructions and resets the internal state.
    pub fn clear(&mut self) {
        self.render_stack.clear();
        self.transform = RawTransform::new();
        self.style = Style::new();
    }

    /// Gets the currently used width and height.
    pub fn get_size(&self) -> (u16, u16) {
        (self.transform.width, self.transform.height)
    }

    /// Returns current size, or defaults if size is zero.
    pub fn get_size_or(&self, width: u16, height: u16) -> (u16, u16) {
        (
            if self.transform.width == 0 {
                width
            } else {
                self.transform.width
            },
            if self.transform.height == 0 {
                height
            } else {
                self.transform.height
            },
        )
    }

    /// Returns size, falling back to parent size if unset.
    pub fn get_size_or_parent(&self) -> (u16, u16) {
        (
            if self.transform.width == 0 {
                self.parent_width
            } else {
                self.transform.width
            },
            if self.transform.height == 0 {
                self.parent_height
            } else {
                self.transform.height
            },
        )
    }

    /// Gets the dimensions of the parent container.
    pub fn get_parent_size(&self) -> (u16, u16) {
        (self.parent_width, self.parent_height)
    }

    /// Sets the dimensions of the parent container.
    pub fn set_parent_size(&mut self, width: u16, height: u16) {
        self.parent_width = width;
        self.parent_height = height;
    }

    /// Returns a mutable reference to the internal raw transform.
    pub fn get_transform_mut(&mut self) -> &mut RawTransform {
        &mut self.transform
    }

    /// Returns a reference to the internal raw transform.
    pub fn get_transform(&self) -> &RawTransform {
        &self.transform
    }

    /// Sets the style for the current render scope.
    pub fn set_style(&mut self, style: Style) {
        self.style = style;
    }

    /// Gets a mutable reference to the style.
    pub fn get_style(&mut self) -> &mut Style {
        &mut self.style
    }
}

impl Debug for RenderScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RenderScope")
    }
}
