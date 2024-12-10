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
            class: "tab",
            text { class: "title", "not started" }
            text { class: "todo", "TODO: Something" }
        }
        div {
            class: "tab",
            text { class: "title", "completed" }
        }
    }
}

pub fn styling() -> Css {
    css! {
        "tab" {
            x: Auto,
            width: 50%,
            outline: true,
        }
        "title" {
            width: Auto,
            color: Red,
        }
        "todo" {
            x: 0 px,
            y: Auto,
            color: Green,
        }
    }
}
