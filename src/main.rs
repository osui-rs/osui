// use std::sync::Arc;

use osui::prelude::*;

pub struct App {}

fn main() {
    // let something = Arc::new(App {});
    while osui::run(&mut app()) {}
}

fn app() -> Element {
    rsx! { styling: Some(styles()),
        button { class: "btn", "Hello, World!" }
    }
}

fn styles() -> Css {
    css! {
        .btn: clicked {
            color: Blue
        }
    }
}
