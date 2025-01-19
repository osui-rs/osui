use osui::prelude::*;

fn main() -> osui::Result<()> {
    let mut con = console::init(true)?;

    con.run(app())?;

    con.end()
}

pub fn app() -> Element {
    rsx! {
        ""
    }
}
