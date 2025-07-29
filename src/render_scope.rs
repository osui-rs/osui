use std::fmt::Debug;

use crate::{
    style::{RawTransform, Transform},
    utils::{self, hex_ansi_bg},
};

#[derive(Clone)]
enum RenderMethod {
    Text(String),
    TextColored(String, u32),
    Rectangle(u16, u16, u32),
}

#[derive(Clone)]
pub struct RenderScope {
    transform: RawTransform,
    render_stack: Vec<RenderMethod>,
    parent_width: u16,
    parent_height: u16,
}

impl RenderScope {
    pub fn new() -> RenderScope {
        RenderScope {
            transform: RawTransform::new(),
            render_stack: Vec::new(),
            parent_width: 0,
            parent_height: 0,
        }
    }

    pub fn set_transform_raw(&mut self, transform: RawTransform) {
        self.transform = transform;
    }

    pub fn set_transform(&mut self, transform: &Transform) {
        transform.use_dimensions(&mut self.transform);
        transform.use_position(self.parent_width, self.parent_height, &mut self.transform);
    }

    pub fn draw_text(&mut self, text: &str) {
        self.render_stack.push(RenderMethod::Text(text.to_string()));
        let (w, h) = utils::str_size(text);
        self.transform.width = self.transform.width.max(w);
        self.transform.height = self.transform.height.max(h);
    }

    pub fn draw_text_colored(&mut self, text: &str, color: u32) {
        self.render_stack
            .push(RenderMethod::TextColored(text.to_string(), color));
        let (w, h) = utils::str_size(text);
        self.transform.width = self.transform.width.max(w);
        self.transform.height = self.transform.height.max(h);
    }

    pub fn draw_rect(&mut self, width: u16, height: u16, color: u32) {
        self.render_stack
            .push(RenderMethod::Rectangle(width, height, color));
        self.transform.width = self.transform.width.max(width);
        self.transform.height = self.transform.height.max(height);
    }

    pub fn draw(&self) {
        for m in &self.render_stack {
            match m {
                RenderMethod::Text(t) => {
                    utils::print(self.transform.x, self.transform.y, t);
                }
                RenderMethod::TextColored(t, c) => utils::print(
                    self.transform.x,
                    self.transform.y,
                    &(utils::hex_ansi(*c) + t),
                ),
                RenderMethod::Rectangle(width, height, color) => {
                    utils::print_liner(
                        self.transform.x,
                        self.transform.y,
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

    pub fn clear(&mut self) {
        self.render_stack.clear();
        self.transform.width = 0;
        self.transform.height = 0;
        self.transform.x = 0;
        self.transform.y = 0;
    }

    pub fn get_size(&self) -> (u16, u16) {
        (self.transform.width, self.transform.height)
    }

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

    pub fn get_parent_size(&self) -> (u16, u16) {
        (self.parent_width, self.parent_height)
    }

    pub fn set_parent_size(&mut self, width: u16, height: u16) {
        self.parent_width = width;
        self.parent_height = height;
    }

    pub fn get_transform_mut(&mut self) -> &mut RawTransform {
        &mut self.transform
    }

    pub fn get_transform(&self) -> &RawTransform {
        &self.transform
    }
}

impl Debug for RenderScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RenderScope")
    }
}
