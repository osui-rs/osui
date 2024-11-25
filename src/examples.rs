use crate::prelude::*;

pub fn examples() -> Element {
    rsx! {
        styling: Some(styling()),
        if (2 == 2) {
            rsx_elem! { button { class: "btn", "Hello, World!" } }
        }
    }
}

pub fn styling() -> Css {
    css! {
        .btn {
            x: 30%,
            y: Auto,
            color: Red,
        }

        .btn: clicked {
            color: Blue,
        }
    }
}
