use crate::{Element, Frame};

pub struct Console(Frame);

pub enum Event {
    Key(crossterm::event::KeyEvent),
    Resize(u16, u16),
    Mouse(crossterm::event::MouseEvent),
    Paste(String),
    FocusGained,
    FocusLost,
}

pub fn init(mouse: bool) -> crate::Result<Console> {
    crossterm::terminal::enable_raw_mode()?;
    crate::utils::clear()?;
    crate::utils::hide_cursor()?;
    if mouse {
        crossterm::execute!(std::io::stdout(), crossterm::event::EnableMouseCapture)?;
    }
    Ok(Console(Frame::new(crossterm::terminal::size()?)))
}

pub fn end() -> crate::Result<()> {
    crossterm::terminal::disable_raw_mode()?;
    crate::utils::clear()?;
    crate::utils::show_cursor()
}

impl Console {
    pub fn draw(&mut self, ui: Element, event: Option<Event>) -> crate::Result<()> {
        self.0.clear()?;
        ui(&mut self.0, event)
    }

    pub fn run(&mut self, ui: Element) -> crate::Result<()> {
        self.draw(ui.clone(), None)?;
        loop {
            let event = read()?;
            if let Event::Resize(w, h) = event {
                self.0.width = w;
                self.0.height = h;
            }
            self.draw(ui.clone(), Some(event))?;
        }
    }
}

pub fn read() -> crate::Result<Event> {
    let event = crossterm::event::read()?;

    Ok(match event {
        crossterm::event::Event::Key(k) => Event::Key(k),
        crossterm::event::Event::Resize(w, h) => Event::Resize(w, h),
        crossterm::event::Event::FocusGained => Event::FocusGained,
        crossterm::event::Event::FocusLost => Event::FocusLost,
        crossterm::event::Event::Mouse(m) => Event::Mouse(m),
        crossterm::event::Event::Paste(p) => Event::Paste(p),
    })
}

pub fn try_read() -> Option<Event> {
    if crossterm::event::poll(std::time::Duration::ZERO).unwrap_or(false) {
        read().ok()
    } else {
        None
    }
}
