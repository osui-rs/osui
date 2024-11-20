use osui::prelude::*;

fn main() {
    while osui::run(&mut app()) {}
}

fn app() -> Element {
    rsx! { styling: Some(styles()),
        button { class: "btn", id: "test", "This is a button!" }
    }
}

fn styles() -> Css {
    css! {
        .btn: clicked {
            color: Blue
        }
    }
}
