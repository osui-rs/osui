use crate::prelude::*;

pub fn examples() -> Element {
    rsx! {
        styling: Some(styling()),

        button { class: "btn", "Click me!\nsdusajuda" }

        text { "Welcome!" }
    }
}

pub fn styling() -> Css {
    css! {
        "btn" {
            outline: true,
            x: Center,
            y: Center,
            width: Auto,
            color: Red,
        }

        "btn": clicked {
            color: Blue,
        }
    }
}
