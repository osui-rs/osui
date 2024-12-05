use crate::prelude::*;

pub fn examples() -> Element {
    rsx! {
        styling: Some(styling()),

        button { class: "btn", on_click: fn(btn: &mut Button, _, document) {
            let count = document.use_state::<u32>("count");
            *count += 1;
            btn.children.set_text(&count.to_string());
            if *count == 10 {
                document.exit();
            }
        }, "Click me!" }

        text { "Welcome!" }

        @count: i32;
    }
}

pub fn todo_app() -> Element {
    rsx! {
        styling: Some(styling()),
        class: "root",
        div {
            class: "first",
            text { class: "title", "not started" }
        }
        div {
            class: "second",
            text { class: "title", "completed" }
        }
    }
}

pub fn styling() -> Css {
    css! {
        "first" {
            x: Auto,
            y: 0 px,
            width: 50%,
        }
        "second" {
            x: Auto,
            y: 0 px,
            width: 50%,
        }
        "root" {
            width: 100%,
        }
        "title" {
            x: Center
        }
    }
}
