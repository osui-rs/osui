use osui::prelude::*;

fn main() -> osui::Result<()> {
    let mut con = console::init()?;

    con.run(app())?;

    console::end()
}

pub fn app() -> Element {
    rsx! {
        "X" (x-Center)
        "Y" (y-Center)

        "Hello, World" (x-Center, y-Center)
    }
}
