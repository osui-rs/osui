pub mod console;
#[cfg(not(feature = "no_elem"))]
pub mod elements;
#[cfg(not(feature = "no_rsx"))]
pub mod rsx;
pub mod state;
pub mod utils;

pub mod prelude {
    pub use crate::*;
    pub use console::Event;
    pub use crossterm::event::{KeyCode, KeyEvent};
}

pub use std::io::Result;

pub type Element = std::sync::Arc<dyn Fn(&mut Frame, Option<console::Event>) -> crate::Result<()>>;

pub trait Widget {
    fn render(&self) -> String;
    fn event(&mut self, event: console::Event) {
        _ = event;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Pos {
    #[allow(non_camel_case_types)]
    auto,
    #[allow(non_camel_case_types)]
    center,
    Num(u16),
}

impl Pos {
    pub fn get(self, auto: u16, width: u16, frame: u16) -> u16 {
        match self {
            Self::auto => auto,
            Self::center => (frame - width) / 2,
            Self::Num(n) => n,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Size {
    Auto,
    Num(u16),
}

impl Size {
    fn get_(self, written: u16) -> u16 {
        match self {
            Self::Auto => written,
            Self::Num(n) => n,
        }
    }

    pub fn get(self, written: u16, _frame: u16) -> u16 {
        self.get_(written)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Area {
    pub width: Size,
    pub height: Size,
    pub x: Pos,
    pub y: Pos,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Frame {
    pub width: u16,
    pub height: u16,
    last_elem: (u16, u16),
}

impl Frame {
    pub fn draw<W>(&mut self, w: &W, props: Area) -> Result<()>
    where
        W: Widget,
    {
        let written = w.render();

        let (ww, wh) = utils::str_size(&written);

        let (width, height) = (
            props.width.get(ww, self.width),
            props.height.get(wh, self.height),
        );

        let (x, y) = (
            props.x.get(self.last_elem.0, width, self.width),
            props.y.get(self.last_elem.1, height, self.height),
        );

        for (i, line) in written.lines().enumerate() {
            if i as u16 > height {
                break;
            }

            println!(
                "\x1b[{};{}H{}",
                y + (i as u16) + 1,
                x + 1,
                line.chars().take(width as usize).collect::<String>()
            );
        }

        self.last_elem.0 = x + width;
        self.last_elem.1 = y + height;

        Ok(())
    }

    pub fn clear(&mut self) -> Result<()> {
        self.last_elem.0 = 0;
        self.last_elem.1 = 0;
        utils::clear()
    }

    pub fn new((width, height): (u16, u16)) -> Self {
        Self {
            width,
            height,
            last_elem: (0, 0),
        }
    }
}

impl Area {
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(feature = "portable")]
    pub fn center_x() -> Self {
        let mut s = Self::default();
        s.x = Pos::Center;
        s
    }

    #[cfg(feature = "portable")]
    pub fn center_y() -> Self {
        let mut s = Self::default();
        s.y = Pos::Center;
        s
    }

    #[cfg(feature = "portable")]
    pub fn center() -> Self {
        let mut s = Self::default();
        s.x = Pos::Center;
        s.y = Pos::Center;
        s
    }

    #[cfg(feature = "portable")]
    pub fn set_width(&mut self, w: Size) -> Self {
        self.width = w;
        *self
    }

    #[cfg(feature = "portable")]
    pub fn set_height(&mut self, h: Size) -> Self {
        self.height = h;
        *self
    }

    #[cfg(feature = "portable")]
    pub fn set_x(&mut self, x: Pos) -> Self {
        self.x = x;
        *self
    }

    #[cfg(feature = "portable")]
    pub fn set_y(&mut self, y: Pos) -> Self {
        self.y = y;
        *self
    }

    #[cfg(feature = "portable")]
    pub fn x_auto(&mut self) -> Self {
        self.x = Pos::Auto;
        *self
    }

    #[cfg(feature = "portable")]
    pub fn y_zero(&mut self) -> Self {
        self.y = Pos::Num(0);
        *self
    }
}

impl Default for Area {
    fn default() -> Self {
        Self {
            width: Size::Auto,
            height: Size::Auto,
            x: Pos::Num(0),
            y: Pos::auto,
        }
    }
}

pub fn use_state<T>(v: T) -> state::State<T> {
    state::State(Box::into_raw(Box::new(v)))
}
