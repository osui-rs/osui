use osui::prelude::*;

fn main() -> osui::Result<()> {
    let mut con = console::init(false)?;

    con.run(app())?;

    con.end()
}

pub fn app() -> Element {
    rsx! {
        btn { on_click: Box::new(|| {
            panic!("Testing");
        }), "Click me" }
    }
}
