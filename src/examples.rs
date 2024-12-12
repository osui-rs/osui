use crate::prelude::*;

pub fn todo_app() -> Element {
    let count = State::new(0);

    rsx! {
        styling: Some(styling()),

        button { on_click: fn(_, _, _) @count {
            let mut count = count.use_state();
            *count += 1;
        }, class: "btn", "Count: {count}" }
    }
}

pub fn styling() -> Css {
    css! {
        "title" {
            color: Green,
            x: Center,
        }

        "container" {
            width: 50%,
        }

        "btn": "clicked" {
            color: Red
        }
    }
}
