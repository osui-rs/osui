use crate::prelude::*;

pub fn todo_app() -> Element {
    let todo = vec!["Foo", "Bar"];

    rsx! {
        // text { class: "title", "Todo list app made with OSUI" }
        // text { class: "title", "{}", "-".repeat(crossterm::terminal::size().unwrap().0 as usize) }
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
        "todo": "hover" {
            color: Blue
        }
        "complete" {
            x: 0 px,
            y: Auto,
            color: Green,
        }
        "complete": "hover" {
            color: Cyan
        }
        "title" {
            x: 0 px,
            y: Auto,
        }
        "title": "hover" {
            color: Blue
        }
    }
}
