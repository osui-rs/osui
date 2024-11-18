use osui::prelude::*;

fn main() {
    while run(&mut app()) {}
}

fn app() -> Element {
    rsx! {
        button { "This is a button!" }
    }
}
