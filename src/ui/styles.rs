//! The `styles` module defines various styling options for UI elements. This includes colors,
//! fonts, padding, margins, and other visual properties that can be applied to UI components
//! to change their appearance.

use std::{collections::HashMap, fmt::Debug};

use dyn_clone::{clone_trait_object, DynClone};

pub trait StyleCore: Debug + Send + Sync + DynClone {
    fn ansi(&self) -> String;
    fn ansi_bg(&self) -> String;
}

clone_trait_object!(StyleCore);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Font {
    None,
    Bold,
    Underline,
    Italic,
    Reverse,
    Strike,
    Mul(Vec<Font>),
}

impl StyleCore for Font {
    fn ansi(&self) -> String {
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
    fn ansi_bg(&self) -> String {
        self.ansi()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Color {
    NoColor,
    Rgb(u8, u8, u8),
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

impl StyleCore for Color {
    fn ansi(&self) -> String {
        String::from(match self {
            Color::NoColor => "",
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

    fn ansi_bg(&self) -> String {
        String::from(match self {
            Color::NoColor => "",
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
}

impl Default for Color {
    fn default() -> Self {
        Color::NoColor
    }
}

impl Default for Font {
    fn default() -> Self {
        Font::None
    }
}

fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
    let hex = hex.trim_start_matches('#');

    let hex = if hex.len() == 3 {
        format!(
            "{}{}{}{}{}{}",
            &hex[0..1],
            &hex[0..1],
            &hex[1..2],
            &hex[1..2],
            &hex[2..3],
            &hex[2..3]
        )
    } else if hex.len() == 6 {
        hex.to_string()
    } else {
        return (255, 255, 255);
    };

    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);

    (r, g, b)
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum StyleName {
    Class(String),
    Id(String),
    Component(String),
}

#[derive(Debug, Clone)]
pub struct StyleElement {
    pub color: Color,
    pub other: HashMap<String, Box<dyn StyleCore>>,
}

impl Default for StyleElement {
    fn default() -> Self {
        StyleElement {
            color: Color::NoColor,
            other: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Style(pub StyleElement, pub HashMap<String, StyleElement>);

impl Default for Style {
    fn default() -> Self {
        Style(StyleElement::default(), HashMap::new())
    }
}

impl Style {
    pub fn write(&self, state: &str, s: &str) -> String {
        if state == "" {
            return self.0.write(s);
        }
        if let Some(style_element) = self.1.get(state) {
            style_element.write(s)
        } else {
            s.to_string()
        }
    }
}

impl StyleElement {
    pub fn write(&self, s: &str) -> String {
        format!("{}{s}\x1b[0m", self.color.ansi())
    }
}

pub type Css = HashMap<StyleName, Style>;
