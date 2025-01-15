pub mod utils;

type Result<T> = std::io::Result<T>;

pub trait Widget {
    fn draw(&self, w: &mut Writer);
}

pub struct Writer(String);

impl Writer {
    pub fn write(&mut self, s: &str) {
        self.0 += s;
    }
}

pub type Element = Box<dyn Widget>;

/////////////////////////////////////////

pub struct Frame {
    pub area: (u16, u16),
    pub x: u16,
    pub y: u16,
    parent: Option<*const Frame>,
}

impl Frame {
    pub fn new(area: (u16, u16), parent: &Frame) -> Frame {
        Frame {
            area,
            x: 0,
            y: 0,
            parent: Some(parent),
        }
    }

    pub fn new_root(area: (u16, u16)) -> Frame {
        Frame {
            area,
            x: 0,
            y: 0,
            parent: None,
        }
    }

    pub fn render<E>(&self, elem: E)
    where
        E: Widget,
    {
        let mut w = Writer(String::new());
        elem.draw(&mut w);
        if let Some(_f) = self.parent {
        } else {
            println!("{}", w.0);
        }
    }
}

pub struct Terminal(Frame);

impl Terminal {
    pub fn new() -> Result<Terminal> {
        Ok(Terminal(Frame::new_root(crossterm::terminal::size()?)))
    }

    pub fn draw<F>(&mut self, f: F) -> Result<()>
    where
        F: Fn(&mut Frame),
    {
        f(&mut self.0);
        Ok(())
    }
}

impl Widget for &str {
    fn draw(&self, w: &mut Writer) {
        w.write(self);
    }
}

pub fn init() -> Result<Terminal> {
    utils::clear()?;
    utils::hide_cursor()?;
    Terminal::new()
}
