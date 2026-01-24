use crate::View;

#[derive(Clone)]
pub enum DrawInstruction {
    Text(Point, String),
    View(Point, View),
    Child(Point, DrawContext),
}

#[derive(Clone)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

#[derive(Clone)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

#[derive(Clone)]
pub struct DrawContext {
    pub available: Size,
    pub allocated: Size,
    pub parent: Point,
    pub drawing: Vec<DrawInstruction>,
}

impl DrawContext {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            available: Size { width, height },
            allocated: Size {
                width: 0,
                height: 0,
            },
            parent: Point { x: 0, y: 0 },
            drawing: Vec::new(),
        }
    }

    pub fn allocate(&mut self, width: u16, height: u16) {
        self.allocated.width += width;
        self.allocated.height += height;
    }

    pub fn draw(&mut self, inst: DrawInstruction) {
        self.drawing.push(inst);
    }

    pub fn draw_text(&mut self, point: Point, text: &str) {
        self.drawing
            .push(DrawInstruction::Text(point, text.to_string()));
    }

    pub fn draw_view(&mut self, point: Point, view: View) {
        let ctx = DrawContext::new(self.allocated.width, self.allocated.height);
        self.drawing.push(DrawInstruction::Child(point, ctx));
    }
}
