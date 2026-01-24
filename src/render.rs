use std::{fmt::Debug, sync::Arc};

pub trait DrawInstruction: Debug {}

#[derive(Debug, Clone)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Clone)]
pub struct DrawContext {
    pub available: Size,
    pub used: Size,
    pub drawing: Vec<Arc<dyn DrawInstruction>>,
}

#[derive(Debug, Clone)]
pub struct RenderContext {}

impl DrawContext {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            available: Size { width, height },
            used: Size {
                width: 0,
                height: 0,
            },
            drawing: Vec::new(),
        }
    }

    pub fn allocate(&mut self, width: u16, height: u16) {
        self.used.width += width;
        self.used.height += height;
    }

    pub fn draw<I: DrawInstruction + 'static>(&mut self, inst: I) {
        self.drawing.push(Arc::new(inst));
    }
}
