use osui::prelude::*;

fn main() -> osui::Result<()> {
    let mut con = console::init()?;

    let (ui, _count) = app();
    con.run(ui).unwrap();

    console::end()
}

pub fn app() -> (Element, state::State<i32>) {
    let count = use_state(0);
    let count1 = count.copy_state();

    (
        rsx! {
            "{count}"
        },
        count1,
    )
}
