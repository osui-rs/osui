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
    /// Red, Green, blue
    Rgb(u8, u8, u8), // RGB color representation.
    Hex(String),
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
            Color::Rgb(r, g, b) => {
                return format!("\x1b[38;2;{r};{g};{b}m");
            }
            Color::Hex(hex) => {
                let (r, g, b) = hex_to_rgb(hex);
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
            Color::Rgb(r, g, b) => {
                return format!("\x1b[48;2;{r};{g};{b}m");
            }
            Color::Hex(hex) => {
                let (r, g, b) = hex_to_rgb(hex);
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

fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
    let hex = hex.trim_start_matches('#');

    // Expand 3-character hex codes to 6 characters
    let hex = if hex.len() == 3 {
        format!(
            "{}{}{}{}{}{}",
            &hex[0..1], &hex[0..1],
            &hex[1..2], &hex[1..2],
            &hex[2..3], &hex[2..3]
        )
    } else if hex.len() == 6 {
        hex.to_string()
    } else {
        return (255, 255, 255); // Return white for invalid hex length
    };

    // Parse each pair of characters as u8
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);

    (r, g, b)
}