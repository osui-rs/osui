use osui::*;

fn main() -> osui::Result<()> {
    utils::init()?;

    let (width, height) = crossterm::terminal::size()?;

    let frame = Frame::new(width, height);

    frame.draw(&"Hello?", Area::new())?;

    utils::read()?;

    utils::end()
}
