use std::sync::Arc;

use osui::{elements::state::use_state, frontend::Rsx, rsx, Screen};

fn main() -> std::io::Result<()> {
    let screen = Screen::new();

    app().draw(&screen);

    screen.run()
}

fn app() -> Rsx {
    let count = use_state(0);

    std::thread::spawn({
        let count = count.clone();
        move || loop {
            std::thread::sleep(std::time::Duration::from_millis(50));
            **count.get() += 1;
        }
    });

    rsx! {
        %count
        @Transform::center();
        "{count}"
    }
}
