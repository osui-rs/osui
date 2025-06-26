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

    pub fn use_position(&self, parent_width: u16, parent_height: u16, raw: &mut RawTransform) {
        self.x.use_position(raw.width, parent_width, &mut raw.x);
        self.y.use_position(raw.height, parent_height, &mut raw.y);
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
            Self::Auto => {}
            Self::Const(n) => *r = *n,
        }
    }
}

impl Position {
    pub fn use_position(&self, size: u16, parent: u16, r: &mut u16) {
        match self {
            Self::Center => *r = (parent - size) / 2,
            Self::Const(n) => *r = *n,
        }
    }
}
