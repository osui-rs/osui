use crate::prelude::*;

pub fn examples() -> Element {
    rsx! {
        styling: Some(styling()),

        button { class: "btn", "Click me!" }

        text { "Welcome!" }
    }
}

pub fn styling() -> Css {
    css! {
        .btn {
            x: Center,
            y: Center,
            color: Red,
        }

        .btn: clicked {
            color: Blue,
        }
    }
}
