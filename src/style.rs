use crate::component;

pub struct RawTransform {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Clone)]
pub enum Position {
    Center,
    Const(u16),
}

#[derive(Debug, Clone)]
pub enum Dimension {
    Auto,
    Const(u16),
}

component!(Transform {
    pub x: Position,
    pub y: Position,
    pub width: Dimension,
    pub height: Dimension,
});

impl Transform {
    pub fn new() -> Transform {
        Transform {
            x: Position::Const(0),
            y: Position::Const(0),
            width: Dimension::Auto,
            height: Dimension::Auto,
        }
    }

    pub fn center() -> Transform {
        Transform {
            x: Position::Center,
            y: Position::Center,
            width: Dimension::Auto,
            height: Dimension::Auto,
        }
    }

    pub fn position(x: Position, y: Position) -> Transform {
        Transform {
            x,
            y,
            width: Dimension::Auto,
            height: Dimension::Auto,
        }
    }

    pub fn use_dimensions(&self, raw: &mut RawTransform) {
        self.width.use_dimension(&mut raw.width);
        self.height.use_dimension(&mut raw.height);
    }
}

impl RawTransform {
    pub fn new() -> RawTransform {
        RawTransform {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }
}

impl Dimension {
    pub fn use_dimension(&self, r: &mut u16) {
        match self {
            Dimension::Auto => {}
            Dimension::Const(n) => *r = *n,
        }
    }
}
