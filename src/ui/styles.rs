/// Represents a style configuration for TUI elements, including background,
/// foreground, outline colors, and font settings for different element states
/// (hovered, clicked, selected).
#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    /// Background color of the element.
    pub bg: Color,
    /// Foreground color of the element.
    pub fg: Color,
    /// Outline color of the element.
    pub outline: Color,
    /// Font style for the element.
    pub font: Font,

    /// Background color when the element is hovered.
    pub hover_bg: Color,
    /// Foreground color when the element is hovered.
    pub hover_fg: Color,
    /// Outline color when the element is hovered.
    pub hover_outline: Color,
    /// Font style when the element is hovered.
    pub hover_font: Font,
    /// Foreground color for the cursor when hovered.
    pub hover_cursor_fg: Color,
    /// Background color for the cursor when hovered.
    pub hover_cursor_bg: Color,

    /// Background color when the element is clicked.
    pub clicked_bg: Color,
    /// Foreground color when the element is clicked.
    pub clicked_fg: Color,
    /// Outline color when the element is clicked.
    pub clicked_outline: Color,
    /// Font style when the element is clicked.
    pub clicked_font: Font,

    /// Background color when the element is selected.
    pub selected_bg: Color,
    /// Foreground color when the element is selected.
    pub selected_fg: Color,
    /// Font style when the element is selected.
    pub selected_font: Font,

    /// Foreground color for the cursor.
    pub cursor_fg: Color,
    /// Background color for the cursor.
    pub cursor_bg: Color,

    /// Indicates if the style is active, affecting its state-based styling.
    pub is_active: bool,
}

impl Default for Style {
    /// Creates a default `Style` instance with `None` for all color and font fields
    /// and inactive (`is_active` set to false).
    fn default() -> Style {
        Style {
            bg: Color::None,
            fg: Color::None,
            outline: Color::None,
            font: Font::None,
            hover_bg: Color::None,
            hover_fg: Color::None,
            hover_outline: Color::None,
            hover_font: Font::None,
            hover_cursor_fg: Color::None,
            hover_cursor_bg: Color::None,
            clicked_bg: Color::None,
            clicked_fg: Color::None,
            clicked_outline: Color::None,
            clicked_font: Font::None,
            selected_bg: Color::None,
            selected_fg: Color::None,
            selected_font: Font::None,
            cursor_fg: Color::None,
            cursor_bg: Color::None,
            is_active: false,
        }
    }
}

impl Style {
    /// Generates the ANSI code for the current style, using the active color
    /// and font if `is_active` is true.
    pub fn get(&self) -> String {
        if self.is_active {
            format!(
                "{}{}{}",
                self.fg.prioritize(&self.hover_fg).ansi(),
                self.bg.prioritize(&self.hover_bg).ansi_bg(),
                self.font.prioritize(&self.hover_font).ansi()
            )
        } else {
            format!(
                "{}{}{}",
                self.fg.ansi(),
                self.bg.ansi_bg(),
                self.font.ansi()
            )
        }
    }

    /// Returns the outline color in ANSI format, applying hover style if active.
    pub fn get_outline(&self) -> String {
        if self.is_active {
            self.outline.prioritize(&self.hover_outline).ansi()
        } else {
            self.outline.ansi()
        }
    }

    /// Retrieves the clicked style ANSI sequence for the element.
    pub fn get_clicked(&self) -> String {
        format!(
            "{}{}{}",
            self.fg.prioritize(&self.clicked_fg).ansi(),
            self.bg.prioritize(&self.clicked_bg).ansi_bg(),
            self.font.prioritize(&self.clicked_font).ansi()
        )
    }

    /// Retrieves the selected style ANSI sequence for the element.
    pub fn get_selected(&self) -> String {
        format!(
            "{}{}{}",
            self.fg.prioritize(&self.selected_fg).ansi(),
            self.bg.prioritize(&self.selected_bg).ansi_bg(),
            self.font.prioritize(&self.selected_font).ansi()
        )
    }

    /// Retrieves the cursor style ANSI sequence for the element, considering hover.
    pub fn get_cursor(&self) -> String {
        if self.is_active {
            format!(
                "{}{}",
                self.cursor_fg.prioritize(&self.hover_cursor_fg).ansi(),
                self.cursor_bg.prioritize(&self.hover_cursor_bg).ansi_bg(),
            )
        } else {
            format!("{}{}", self.cursor_fg.ansi(), self.cursor_bg.ansi_bg())
        }
    }

    /// Writes the given string with the active style's ANSI sequences.
    pub fn write(&self, s: &str) -> String {
        format!("{}{}\x1b[0m", self.get(), s)
    }

    /// Writes a string with the outline color applied.
    pub fn write_outline(&self, s: &str) -> String {
        format!("{}{}\x1b[0m", self.get_outline(), s)
    }

    /// Writes a string with the clicked style applied.
    pub fn write_clicked(&self, s: &str) -> String {
        format!("{}{}\x1b[0m", self.get_clicked(), s)
    }

    /// Writes a string with the selected style applied.
    pub fn write_selected(&self, s: &str) -> String {
        format!("{}{}\x1b[0m", self.get_selected(), s)
    }

    /// Writes a string with the cursor style applied.
    pub fn write_cursor(&self, s: &str) -> String {
        format!("{}{}\x1b[0m", self.get_cursor(), s)
    }

    /// Clones the style and sets `is_active` based on the given state.
    pub fn use_style(&self, state: &usize) -> Style {
        let mut style = self.clone();
        style.is_active = *state == 1;
        style
    }
}

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
