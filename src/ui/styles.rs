/// Represents different font styles that can be applied to TUI elements.
#[derive(Debug, Clone, PartialEq)]
pub enum Font {
    None,
    Bold,
    Underline,
    Italic,
    Reverse,
    Strike,
    Mul(Vec<Font>), // Allows combining multiple font styles.
}

impl Font {
    /// Converts the font style to an ANSI sequence.
    pub fn ansi(&self) -> String {
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

    /// Returns the prioritized font between `self` and `secondary`, favoring `secondary`
    /// if it is not `None`.
    pub fn prioritize<'a>(&'a self, secondary: &'a Font) -> &Font {
        if secondary == &Font::None {
            self
        } else {
            secondary
        }
    }
}

/// Represents color options for elements, supporting both named colors and RGB values.
#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    None,
    Rgb(u8, u8, u8), // RGB color representation.
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
    /// Converts the color to an ANSI foreground sequence.
    pub fn ansi(&self) -> String {
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

    /// Converts the color to an ANSI background sequence.
    pub fn ansi_bg(&self) -> String {
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

    /// Returns the prioritized color between `self` and `secondary`, favoring `secondary`
    /// if it is not `None`.
    pub fn prioritize<'a>(&'a self, secondary: &'a Color) -> &Color {
        if secondary == &Color::None {
            self
        } else {
            secondary
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::None
    }
}

impl Default for Font {
    fn default() -> Self {
        Font::None
    }
}