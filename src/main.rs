use osui::prelude::*;

fn main() -> osui::Result<()> {
    let con = console::init()?;

    let (ui, count) = app();

    loop {
        let count = count.copy_state();
        let mut count1 = count.copy_state();
        con.draw(std::sync::Arc::clone(&ui))?;

        if let Event::Key(KeyEvent { code, .. }) = console::read()? {
            if code == KeyCode::Char('q') {
                break;
            }
        }
        count1 += 1;
    }

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
