use crate::prelude::*;

pub fn examples() -> Element {
    rsx! {
        styling: Some(styling()),
        button { class: "btn", "Hello, World!" }
    }
}

pub fn styling() -> Css {
    css! {
        .btn {
            x: Center,
            y: Auto,
            color: Red,
        }

        .btn: clicked {
            color: Blue,
        }
    }
}
