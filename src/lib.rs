pub mod utils;
pub mod widgets;

pub use std::io::Result;

pub trait Widget {
    fn render(&self) -> String;
}

#[derive(Debug, Clone, Copy)]
pub struct Area {
    width: u16,
    height: u16,
    x: u16,
    y: u16,
    center_x: bool,
    center_y: bool,
    width_auto: bool,
    height_auto: bool,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Frame {
    pub area: Area,
}

impl Frame {
    pub fn draw<W>(&self, w: &W, mut props: Area) -> Result<()>
    where
        W: Widget,
    {
        let written = w.render();

        let (ww, wh) = utils::str_size(&written);

        if props.width_auto {
            props.width = ww;
        }
        if props.height_auto {
            props.height = wh;
        }

        if props.width > self.area.width {
            props.width -= props.width - self.area.width;
        }

        if props.width > self.area.width {
            props.width -= props.width - self.area.width;
        }

        if props.center_x {
            props.x = (self.area.width - ww) / 2;
        }
        if props.center_y {
            props.y = (self.area.height - wh) / 2;
        }

        for (i, line) in written.lines().enumerate() {
            if i as u16 > props.height {
                break;
            }

            println!(
                "\x1b[{};{}H{}",
                props.x + (i as u16) + 1,
                props.y + 1,
                line.chars().take(props.width as usize).collect::<String>()
            );
        }

        Ok(())
    }

    pub fn new(width: u16, height: u16) -> Self {
        let mut f = Self::default();
        f.area.width(width);
        f.area.height(height);
        f
    }
}

impl Area {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn center_x(&mut self) -> Self {
        self.center_x = true;
        *self
    }

    pub fn center_y(&mut self) -> Self {
        self.center_y = true;
        *self
    }

    pub fn center(&mut self) -> Self {
        self.center_x = true;
        self.center_y = true;
        *self
    }

    pub fn width(&mut self, w: u16) -> Self {
        self.width = w;
        self.width_auto = false;
        *self
    }

    pub fn height(&mut self, h: u16) -> Self {
        self.height = h;
        self.height_auto = false;
        *self
    }
}

impl Default for Area {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            x: 0,
            y: 0,
            center_x: false,
            center_y: false,
            width_auto: true,
            height_auto: true,
        }
    }
}
