use osui::prelude::*;

fn main() -> osui::Result<()> {
    let mut con = console::init(false)?;

    con.run(app())?;

    console::end()
}

pub fn app() -> Element {
    rsx! {
        test {} ()
    }
}

pub fn test() -> Element {
    rsx! {
        "ABC" (x-center y-center)
    }
}
