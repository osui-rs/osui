mod demos;
use osui::prelude::*;

fn main() -> std::io::Result<()> {
    let screen = Screen::new();
    screen.extension(InputExtension);
    screen.extension(RelativeFocusExtension::new());

    demos::app(screen.clone()).draw(&screen);

    screen.run()
}
