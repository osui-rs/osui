//! The `styles` module defines various styling options for UI elements. This includes colors,
//! fonts, padding, margins, and other visual properties that can be applied to UI components
//! to change their appearance.

use std::{collections::HashMap, fmt::Debug};

use dyn_clone::{clone_trait_object, DynClone};

//////////////////////////////////////////////////////////////////////////////////////////////////
// Type aliases
//////////////////////////////////////////////////////////////////////////////////////////////////

pub type Css = HashMap<StyleName, StyleElement>;

//////////////////////////////////////////////////////////////////////////////////////////////////
// Traits
//////////////////////////////////////////////////////////////////////////////////////////////////

pub trait StyleCore: Debug + Send + Sync + DynClone {
    fn ansi(&self) -> String;
    fn ansi_bg(&self) -> String;
}
clone_trait_object!(StyleCore);

//////////////////////////////////////////////////////////////////////////////////////////////////
// Enums
//////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Font {
    Bold,
    Underline,
    Italic,
    Reverse,
    Strike,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Number {
    Px(u16),
    Pe(u16),
    Center,
    Auto,
    Default,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum StyleName {
    Class(String),
    ClassState(String, String),
}

//////////////////////////////////////////////////////////////////////////////////////////////////
// Structs
//////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct StyleElement {
    pub color: (bool, Color),
    pub background: (bool, Color),
    pub outline_color: (bool, Color),
    pub font: (bool, Vec<Font>),
    pub x: (bool, Number),
    pub y: (bool, Number),
    pub width: (bool, Number),
    pub height: (bool, Number),
    pub visible: (bool, bool),
    pub outline: (bool, bool),
}

#[derive(Debug, Clone)]
pub struct Style(
    pub StyleElement,
    pub HashMap<String, StyleElement>,
    pub String,
);

//////////////////////////////////////////////////////////////////////////////////////////////////
// Implementations
//////////////////////////////////////////////////////////////////////////////////////////////////

impl Default for Style {
    fn default() -> Self {
        Style(StyleElement::default(), HashMap::new(), String::new())
    }
}

impl Style {
    pub fn get(mut self, hover: bool) -> StyleElement {
        if let Some(style_element) = self.1.get(&self.2) {
            self.0.merge(style_element);
            self.0
        } else if hover {
            if let Some(style_element) = self.1.get("hover") {
                self.0.merge(style_element);
                self.0
            } else {
                self.0
            }
        } else {
            self.0
        }
    }

    pub fn set_state(&mut self, state: &str) {
        self.2 = state.to_string();
    }
}

impl StyleElement {
    pub fn write(&self, s: &str) -> String {
        s.split('\n')
            .into_iter()
            .map(|p| {
                format!(
                    "{}{}{}{p}\x1b[0m",
                    self.color.1.ansi(),
                    self.background.1.ansi_bg(),
                    self.font.1.ansi()
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
    pub fn write_outline(&self, outline: &str) -> String {
        format!("{}{outline}\x1b[0m", self.outline_color.1.ansi(),)
    }
}

impl Default for StyleElement {
    fn default() -> Self {
        StyleElement {
            color: (false, Color::NoColor),
            background: (false, Color::NoColor),
            font: (false, Vec::new()),
            outline_color: (false, Color::NoColor),
            x: (false, Number::Default),
            y: (false, Number::Px(0)),
            width: (false, Number::Default),
            height: (false, Number::Default),
            visible: (false, true),
            outline: (false, false),
        }
    }
}

impl StyleElement {
    pub fn merge(&mut self, upper: &Self) {
        if upper.color.0 {
            self.color = upper.color.clone();
        }
        if upper.background.0 {
            self.background = upper.background.clone();
        }
        if upper.font.0 {
            self.font = upper.font.clone();
        }
        if upper.outline_color.0 {
            self.outline_color = upper.outline_color.clone();
        }
        if upper.x.0 {
            self.x = upper.x.clone();
        }
        if upper.y.0 {
            self.y = upper.y.clone();
        }
        if upper.width.0 {
            self.width = upper.width.clone();
        }
        if upper.height.0 {
            self.height = upper.height.clone();
        }
        if upper.visible.0 {
            self.visible = upper.visible.clone();
        }
        if upper.outline.0 {
            self.outline = upper.outline.clone();
        }
    }
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

impl StyleCore for Font {
    fn ansi(&self) -> String {
        String::from(match self {
            Font::Bold => "\x1b[1m",
            Font::Underline => "\x1b[4m",
            Font::Italic => "\x1b[3m",
            Font::Reverse => "\x1b[7m",
            Font::Strike => "\x1b[9m",
        })
    }
    fn ansi_bg(&self) -> String {
        self.ansi()
    }
}

impl StyleCore for Vec<Font> {
    fn ansi(&self) -> String {
        let mut a = String::new();
        for i in self {
            a += i.ansi().as_str();
        }
        a
    }

    fn ansi_bg(&self) -> String {
        let mut a = String::new();
        for i in self {
            a += i.ansi_bg().as_str();
        }
        a
    }
}

impl Number {
    pub fn as_size_raw(&self, frame_size: u16) -> u16 {
        match self {
            Number::Px(s) => *s,
            Number::Pe(pe) => (frame_size * pe) / 100,
            _ => frame_size,
        }
    }
    pub fn as_size(&self, written: u16, frame_size: u16, outline: bool) -> u16 {
        if *self == Number::Auto || *self == Number::Default {
            written
        } else if outline {
            self.as_size_raw(frame_size) - 2
        } else {
            self.as_size_raw(frame_size)
        }
    }
    pub fn as_position(&self, used: &u16, frame_size: u16) -> u16 {
        match self {
            crate::ui::Number::Px(px) => *px,
            crate::ui::Number::Pe(pe) => (frame_size * pe) / 100,
            crate::ui::Number::Center => (frame_size) / 2,
            crate::ui::Number::Auto | crate::ui::Number::Default => *used,
        }
    }
}
