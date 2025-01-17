use osui::*;

fn main() -> osui::Result<()> {
    let con = console::init()?;

    loop {
        con.draw(|frame| -> Result<()> {
            frame.draw(&"testing", Area::center())?;
            frame.draw(&"\x1b[32mX\x1b[0m", Area::center_x())?;
            frame.draw(&"\x1b[31mY\x1b[0m", Area::center_y())?;
            Ok(())
        })?;
    }

    console::end()
}
