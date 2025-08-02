mod demos;
use osui::prelude::*;

fn main() -> std::io::Result<()> {
    let screen = Screen::new();

    demos::app().draw(&screen);

    screen.run()
}
