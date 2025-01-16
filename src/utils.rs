use std::io::Write;

pub fn clear() -> crate::Result<()> {
    print!("\x1B[2J\x1B[H");
    std::io::stdout().flush()
}

pub fn hide_cursor() -> crate::Result<()> {
    print!("\x1b[?25l");
    std::io::stdout().flush()
}

pub fn show_cursor() -> crate::Result<()> {
    print!("\x1B[?25h");
    std::io::stdout().flush()
}

pub fn flush() -> crate::Result<()> {
    std::io::stdout().flush()
}

pub fn init() -> crate::Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    clear()?;
    hide_cursor()
}

pub fn end() -> crate::Result<()> {
    crossterm::terminal::disable_raw_mode()?;
    clear()?;
    show_cursor()
}

pub fn str_size(s: &str) -> (u16, u16) {
    let mut height = 1;
    let mut max_width = 0;
    let mut current_width = 0;

    for b in s.bytes() {
        if b == b'\n' {
            height += 1;
            max_width = max_width.max(current_width);
            current_width = 0;
        } else {
            current_width += 1;
        }
    }

    max_width = max_width.max(current_width);

    (max_width, height)
}

pub enum Event {
    Key(crossterm::event::KeyEvent),
    Resize(u16, u16),
    Mouse(crossterm::event::MouseEvent),
    Paste(String),
    FocusGained,
    FocusLost,
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
