use crate::View;

#[derive(Clone)]
pub enum DrawInstruction {
    Text(Point, String),
    View(Area, View),
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
pub struct Area {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

#[derive(Clone)]
pub struct DrawContext {
    pub area: Area,
    pub allocated: Size,
    pub drawing: Vec<DrawInstruction>,
}

impl DrawContext {
    pub fn new(area: Area) -> Self {
        Self {
            area,
            allocated: Size {
                width: 0,
                height: 0,
            },
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

    pub fn draw_view(&mut self, area: Area, view: View) {
        self.drawing.push(DrawInstruction::View(area, view));
    }
}
