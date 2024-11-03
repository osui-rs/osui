use osui::{rsx, ui::*, App};

fn main() {
    let element = rsx! {
        button { "Hello, World!" }
    };

    let mut app_screen = App::from(element);
    app_screen.run();
}
