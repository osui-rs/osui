use crate::component;

#[derive(Debug, Clone)]
pub struct RawTransform {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub px: u16,
    pub py: u16,
}

#[derive(Debug, Clone)]
pub enum Position {
    Const(u16),
    Center,
    End,
}

#[derive(Debug, Clone)]
pub enum Dimension {
    Full,
    Content,
    Const(u16),
}

#[derive(Debug, Clone)]
pub enum Background {
    NoBackground,
    Outline(u32),
    RoundedOutline(u32),
    Solid(u32),
}

component!(Transform {
    pub x: Position,
    pub y: Position,
    pub mx: i32,
    pub my: i32,
    pub px: u16,
    pub py: u16,
    pub width: Dimension,
    pub height: Dimension,
});

component!(Style {
    pub background: Background,
    pub foreground: Option<u32>,
});

impl Style {
    pub fn new() -> Self {
        Self {
            background: Background::NoBackground,
            foreground: None,
        }
    }
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            x: Position::Const(0),
            y: Position::Const(0),
            mx: 0,
            my: 0,
            px: 0,
            py: 0,
            width: Dimension::Content,
            height: Dimension::Content,
        }
    }

    pub fn center() -> Transform {
        Transform {
            x: Position::Center,
            y: Position::Center,
            mx: 0,
            my: 0,
            px: 0,
            py: 0,
            width: Dimension::Content,
            height: Dimension::Content,
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

    pub fn padding(mut self, x: u16, y: u16) -> Self {
        self.px = x;
        self.py = y;
        self
    }

    pub fn dimensions(mut self, width: u16, height: u16) -> Self {
        self.width = Dimension::Const(width);
        self.height = Dimension::Const(height);
        self
    }

    pub fn use_dimensions(&self, parent_width: u16, parent_height: u16, raw: &mut RawTransform) {
        self.width.use_dimension(parent_width, &mut raw.width);
        self.height.use_dimension(parent_height, &mut raw.height);
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
            px: 0,
            py: 0,
        }
    }
}

impl Dimension {
    pub fn use_dimension(&self, parent: u16, r: &mut u16) {
        match self {
            Self::Full => *r = parent,
            Self::Content => {}
            Self::Const(n) => *r = *n,
        }
    }
}

impl Position {
    pub fn use_position(&self, size: u16, parent: u16, m: i32, r: &mut u16) {
        let base = match self {
            Self::Center => {
                if parent < size {
                    return;
                }
                (parent - size) / 2
            }
            Self::Const(n) => *n,
            Self::End => {
                if parent < size {
                    return;
                }
                parent - size
            }
        };

        let adjusted = if m >= 0 {
            base.checked_add(m as u16)
        } else {
            base.checked_sub((-m) as u16)
        };

        if let Some(val) = adjusted {
            *r = val;
        }
    }
}

impl From<u16> for Position {
    fn from(value: u16) -> Self {
        Self::Const(value)
    }
}

impl From<u16> for Dimension {
    fn from(value: u16) -> Self {
        Self::Const(value)
    }
}
