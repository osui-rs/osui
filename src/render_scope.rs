//! Rendering logic and layout scope management for OSUI.
//!
//! `RenderScope` is responsible for handling transformations, parent-child dimensions,
//! stacking draw operations, and rendering styled output to the terminal.
//!
//! This module allows widgets to accumulate rendering commands (text, shapes, colors),
//! and then flush them to the screen with correct styling and positioning.
//!
//! Used internally by OSUI's layout and rendering system.

use std::{fmt::Debug, sync::Arc};

use crate::{
    prelude::{Context, Handler},
    style::{RawTransform, Style, Transform},
    utils::{self, hex_ansi_bg},
    widget::Widget,
    NoRender, RenderWrapperEvent,
};

pub trait ElementRenderer {
    /// Called right after the `after_render` function is called
    #[allow(unused)]
    fn before_draw(&mut self, scope: &mut RenderScope, widget: &Arc<Widget>) {}
}

/// Represents a single render instruction.
#[derive(Clone)]
enum RenderMethod {
    /// Plain text rendering at current transform.
    Text(u16, u16, String),
    /// Plain text rendering at current transform with the bg and fg swapped.
    TextInverted(u16, u16, String),
    /// Text rendered with a specific 24-bit color.
    TextColored(u16, u16, String, u32),
    /// A filled rectangle of a given size and background color.
    Rectangle(u16, u16, u16, u16, u32),
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

pub struct RenderContext(Context, bool);

impl RenderContext {
    pub fn new(c: &Context, focused: bool) -> Self {
        Self(c.clone(), focused)
    }

    pub fn is_focused(&self) -> bool {
        self.1
    }

    pub fn render(&self, w: &Arc<Widget>, scope: &mut RenderScope) {
        self.0.render(w, scope);
    }

    pub fn after_render(&self, w: &Arc<Widget>, scope: &mut RenderScope) {
        self.0.after_render(w, scope);
    }

    pub fn get_context(&self) -> &Context {
        &self.0
    }
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
    pub fn draw_text(&mut self, x: u16, y: u16, text: &str) {
        self.render_stack
            .push(RenderMethod::Text(x, y, text.to_string()));
        let (w, h) = utils::str_size(text);
        self.transform.width = self.transform.width.max(w);
        self.transform.height = self.transform.height.max(h);
    }

    /// Adds a text draw instruction.
    pub fn draw_text_inverted(&mut self, x: u16, y: u16, text: &str) {
        self.render_stack
            .push(RenderMethod::TextInverted(x, y, text.to_string()));
        let (w, h) = utils::str_size(text);
        self.transform.width = self.transform.width.max(w);
        self.transform.height = self.transform.height.max(h);
    }

    /// Adds a colored text draw instruction.
    pub fn draw_text_colored(&mut self, x: u16, y: u16, text: &str, color: u32) {
        self.render_stack
            .push(RenderMethod::TextColored(x, y, text.to_string(), color));
        let (w, h) = utils::str_size(text);
        self.transform.width = self.transform.width.max(w);
        self.transform.height = self.transform.height.max(h);
    }

    /// Adds a background rectangle draw instruction.
    pub fn draw_rect(&mut self, x: u16, y: u16, width: u16, height: u16, color: u32) {
        self.render_stack
            .push(RenderMethod::Rectangle(x, y, width, height, color));
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
                        (self.parent_width, self.parent_height),
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
                        (self.parent_width, self.parent_height),
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
                (self.parent_width, self.parent_height),
            ),
        }

        for m in &self.render_stack {
            match m {
                RenderMethod::Text(x, y, t) => {
                    if *y + self.transform.y >= self.parent_height {
                        continue;
                    }
                    if let Some(c) = self.style.foreground {
                        utils::print_liner(
                            self.transform.x + self.transform.px + x,
                            self.transform.y + self.transform.py + y,
                            &utils::hex_ansi(c),
                            t,
                            (self.parent_width, self.parent_height),
                        );
                    } else {
                        utils::print(
                            self.transform.x + self.transform.px,
                            self.transform.y + self.transform.py,
                            t,
                            (self.parent_width, self.parent_height),
                        );
                    }
                }
                RenderMethod::TextInverted(x, y, t) => {
                    if let Some(c) = self.style.foreground {
                        utils::print_liner(
                            self.transform.x + self.transform.px + x,
                            self.transform.y + self.transform.py + y,
                            &utils::hex_ansi_bg(c),
                            t,
                            (self.parent_width, self.parent_height),
                        );
                    } else {
                        utils::print(
                            self.transform.x + self.transform.px,
                            self.transform.y + self.transform.py,
                            t,
                            (self.parent_width, self.parent_height),
                        );
                    }
                }
                RenderMethod::TextColored(x, y, t, c) => utils::print_liner(
                    self.transform.x + self.transform.px + x,
                    self.transform.y + self.transform.py + y,
                    &utils::hex_ansi(*c),
                    t,
                    (self.parent_width, self.parent_height),
                ),
                RenderMethod::Rectangle(x, y, width, height, color) => utils::print_liner(
                    self.transform.x + self.transform.px + x,
                    self.transform.y + self.transform.py + y,
                    &hex_ansi_bg(*color),
                    &std::iter::repeat(" ".repeat(*width as usize))
                        .take(*height as usize)
                        .collect::<Vec<_>>()
                        .join("\n"),
                    (self.parent_width, self.parent_height),
                ),
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

    pub fn render_widget(
        &mut self,
        renderer: &mut dyn ElementRenderer,
        ctx: &crate::extensions::Context,
        widget: &std::sync::Arc<crate::widget::Widget>,
    ) -> bool {
        if widget.get::<NoRender>().is_some() {
            widget.auto_refresh();
            return false;
        }

        if let Some(wrapper) = widget.get::<Handler<RenderWrapperEvent>>() {
            wrapper.call(widget, &RenderWrapperEvent(self));
        } else {
            self.clear();

            let render_context = RenderContext::new(ctx, widget.is_focused());

            if let Some(style) = widget.get() {
                self.set_style(style);
            }
            if let Some(t) = widget.get() {
                self.set_transform(&t);
            }

            widget.get_elem().render(self, &render_context);
            ctx.render(widget, self);

            if let Some(t) = widget.get() {
                self.set_transform(&t);
            }

            renderer.before_draw(self, widget);

            self.draw();

            widget.get_elem().after_render(self, &render_context);
            ctx.after_render(widget, self);
        }

        widget.auto_refresh();

        true
    }
}

impl Debug for RenderScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RenderScope")
    }
}
