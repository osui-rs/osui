use osui::prelude::*;

fn main() {
    while osui::run(&mut app()) {}
}

fn app() -> Element {
    rsx! { styling: Some(styles()),
        button { class: "btn", "This is a button!" }
    }
}

fn styles() -> Css {
    css! {
        .btn::click {
            color: Blue
        }
    }
}
