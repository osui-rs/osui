use osui::prelude::*;

fn main() -> osui::Result<()> {
    let mut con = console::init(true)?;

    con.run(app())?;

    con.end()
}

pub fn app() -> Element {
    let count = use_state(0);

    rsx! {
        if (count == 0) {
            "ok"
        }

        "{count}"
    }
}
