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
