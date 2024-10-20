#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    pub bg: Color,
    pub fg: Color,
    pub outline: Color,
    pub font: Font,

    pub hover_bg: Color,
    pub hover_fg: Color,
    pub hover_outline: Color,
    pub hover_font: Font,

    pub is_active: bool,
}

impl Style {
    pub fn new() -> Style {
        Style {
            bg: Color::None,
            fg: Color::None,
            outline: Color::None,
            font: Font::None,

            hover_bg: Color::None,
            hover_fg: Color::None,
            hover_outline: Color::None,
            hover_font: Font::None,

            is_active: false,
        }
    }

    pub fn get(self) -> String {
        if self.is_active {
            String::from(
                self.fg.prioritize(self.hover_fg).ansi()
                    + &self.bg.prioritize(self.hover_bg).ansi_bg()
                    + &self.font.prioritize(self.hover_font).ansi(),
            )
        } else {
            String::from(self.fg.ansi() + &self.bg.ansi_bg() + &self.font.ansi())
        }
    }

    pub fn get_outline(self) -> String {
        if self.is_active {
            self.outline.prioritize(self.hover_outline).ansi()
        } else {
            self.outline.ansi()
        }
    }

    pub fn write(self, s: &str) -> String {
        format!("{}{}", self.get(), s)
    }

    pub fn write_outline(self, s: &str) -> String {
        format!("{}{}", self.get_outline(), s)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Font {
    None,
    Bold,
    Underline,
    Italic,
    Reverse,
    Strike,
    Mul(Vec<Font>),
}

impl Font {
    pub fn ansi(self) -> String {
        String::from(match self {
            Font::None => "",
            Font::Bold => "\x1b[1m",
            Font::Underline => "\x1b[4m",
            Font::Italic => "\x1b[3m",
            Font::Reverse => "\x1b[7m",
            Font::Strike => "\x1b[9m",
            Font::Mul(v) => {
                let mut s = String::new();
                for n in v {
                    s += n.ansi().as_str();
                }
                return s;
            }
        })
    }

    pub fn prioritize(self, secondary: Font) -> Font {
        if secondary == Font::None {
            self
        } else {
            secondary
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    None,
    Rgb(u8, u8, u8),
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Color {
    pub fn ansi(self) -> String {
        String::from(match self {
            Color::None => "",
            Self::Rgb(r, g, b) => {
                return format!("\x1b[38;2;{r};{g};{b}m");
            }
            Color::Black => "\x1b[30m",
            Color::Red => "\x1b[31m",
            Color::Green => "\x1b[32m",
            Color::Yellow => "\x1b[33m",
            Color::Blue => "\x1b[34m",
            Color::Magenta => "\x1b[35m",
            Color::Cyan => "\x1b[36m",
            Color::White => "\x1b[37m",
        })
    }

    pub fn ansi_bg(self) -> String {
        String::from(match self {
            Color::None => "",
            Self::Rgb(r, g, b) => {
                return format!("\x1b[48;2;{r};{g};{b}m");
            }
            Color::Black => "\x1b[40m",
            Color::Red => "\x1b[41m",
            Color::Green => "\x1b[42m",
            Color::Yellow => "\x1b[43m",
            Color::Blue => "\x1b[44m",
            Color::Magenta => "\x1b[45m",
            Color::Cyan => "\x1b[46m",
            Color::White => "\x1b[47m",
        })
    }

    pub fn prioritize(self, secondary: Color) -> Color {
        if secondary == Color::None {
            self
        } else {
            secondary
        }
    }
}
