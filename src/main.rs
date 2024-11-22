use osui::prelude::*;

pub struct App {}

fn main() {
    while osui::run(&mut app()) {}
}

fn app() -> Element {
    rsx! { styling: Some(styles()),
        button { class: "btn", on_click: Handler::new(|_, _, document| {

            if let Some(txt) = document.get_element_by_id::<Text>("txt") {
                txt.children.set_text("edited");
            }

        }), "Hello, World!" }
        text { id: "txt", "Some text!" }
    }
}

fn styles() -> Css {
    css! {
        .btn: clicked {
            color: Blue
        }
        .btn: hover {
            color: Red
        }
    }
}
