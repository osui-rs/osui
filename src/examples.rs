use crate::prelude::*;

pub fn todo_app() -> Element {
    let todo = vec!["Foo", "Bar"];

    rsx! {
        for (t in todo) {
            ersx!( button{ class: "todo", on_click: fn(btn: &mut Button, _, _) {
                btn.class = if btn.class == "todo" {
                    "complete"
                } else {
                    "todo"
                };
            }, "{t}" } )
        }
    }
}

pub fn todo_styling() -> Css {
    css! {
        "todo" {
            x: 0 px,
            y: Auto,
            color: Red,
        }
        "complete" {
            x: 0 px,
            y: Auto,
            color: Green,
        }
    }
}
