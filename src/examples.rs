use crate::prelude::*;

pub fn event_logger() -> Element {
    rsx! {
        styling: Some(styling()),
        button { class: "txt", "Hello, World!" }
    }
}

pub fn styling() -> Css {
    css! {
        .txt {
            x: Center,
            y: Center,
            color: Red,
        }
        
        .txt: clicked {
            color: Blue,
        }
    }
}
