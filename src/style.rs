use crate::component;

#[derive(Clone)]
pub struct RawTransform {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Clone)]
pub enum Position {
    Const(u16),
    Center,
    End,
}

#[derive(Debug, Clone)]
pub enum Dimension {
    Auto,
    Const(u16),
}

component!(Transform {
    pub x: Position,
    pub y: Position,
    pub mx: i32,
    pub my: i32,
    pub width: Dimension,
    pub height: Dimension,
});

impl Transform {
    pub fn new() -> Transform {
        Transform {
            x: Position::Const(0),
            y: Position::Const(0),
            mx: 0,
            my: 0,
            width: Dimension::Auto,
            height: Dimension::Auto,
        }
    }

    pub fn center() -> Transform {
        Transform {
            x: Position::Center,
            y: Position::Center,
            mx: 0,
            my: 0,
            width: Dimension::Auto,
            height: Dimension::Auto,
        }
    }

    pub fn bottom(mut self) -> Self {
        self.y = Position::End;
        self
    }

    pub fn right(mut self) -> Self {
        self.x = Position::End;
        self
    }

    pub fn margin(mut self, x: i32, y: i32) -> Self {
        self.mx = x;
        self.my = y;
        self
    }

    pub fn dimensions(mut self, width: u16, height: u16) -> Self {
        self.width = Dimension::Const(width);
        self.height = Dimension::Const(height);
        self
    }

    pub fn use_dimensions(&self, raw: &mut RawTransform) {
        self.width.use_dimension(&mut raw.width);
        self.height.use_dimension(&mut raw.height);
    }

    pub fn use_position(&self, parent_width: u16, parent_height: u16, raw: &mut RawTransform) {
        self.x
            .use_position(raw.width, parent_width, self.mx, &mut raw.x);
        self.y
            .use_position(raw.height, parent_height, self.my, &mut raw.y);
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
    pub fn use_position(&self, size: u16, parent: u16, m: i32, r: &mut u16) {
        match self {
            Self::Center => *r = (parent - size) / 2,
            Self::Const(n) => *r = *n,
            Self::End => *r = parent - size,
        }

        if m > 0 {
            *r += m as u16;
        } else {
            *r -= -m as u16;
        }
    }
}
