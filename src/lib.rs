use std::sync::Arc;

pub mod component;
pub mod state;

pub mod prelude {
    pub use crate::component::*;
    pub use crate::state::*;
    pub use crate::{DrawContext, View};
}

pub type View = Arc<dyn Fn(&mut DrawContext) + Send + Sync>;

#[derive(Debug, Clone)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Clone)]
pub struct DrawContext {
    available: Size,
    used: Size,
}

impl DrawContext {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            available: Size { width, height },
            used: Size {
                width: 0,
                height: 0,
            },
        }
    }

    pub fn available(&self) -> &Size {
        &self.available
    }

    pub fn used(&self) -> &Size {
        &self.used
    }

    pub fn allocate(&mut self, width: u16, height: u16) {
        self.used.width += width;
        self.used.height += height;
    }
}
