use osui::prelude::*;

fn main() -> Result<()> {
    let mut con = init(true)?;

    con.run(app())?;

    con.end()
}

pub fn app() -> Element {
    rsx! {
        btn { "Click me" } (x-center y-center)
    }
}
