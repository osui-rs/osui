use crate::{
    style::{RawTransform, Transform},
    utils,
};

enum RenderMethod {
    Text(String),
}

pub struct RenderScope {
    transform: RawTransform,
    render_stack: Vec<RenderMethod>,
    width: u16,
    height: u16,
    parent_width: u16,
    parent_height: u16,
}

impl RenderScope {
    pub fn new() -> RenderScope {
        RenderScope {
            transform: RawTransform::new(),
            render_stack: Vec::new(),
            width: 0,
            height: 0,
            parent_width: 0,
            parent_height: 0,
        }
    }

    pub fn set_transform(&mut self, transform: &Transform) {
        transform.use_dimensions(&mut self.transform);
    }

    pub fn draw_text(&mut self, text: &str) {
        self.render_stack.push(RenderMethod::Text(text.to_string()));
        let (w, h) = utils::str_size(text);
        self.width = std::cmp::max(self.width, w);
        self.height = std::cmp::max(self.height, h);
    }

    pub fn draw(&self) {
        for m in &self.render_stack {
            match m {
                RenderMethod::Text(t) => {
                    println!("\x1b[{};{}H{t}", self.transform.y + 1, self.transform.x + 1)
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.render_stack.clear();
    }
}
