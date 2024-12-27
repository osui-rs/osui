use osui::prelude::*;

fn main() {
    launch!(App);
}

#[component]
pub fn App() {
    ersx! {
        text { style: style! {
            x: 30 px,
            y: 4 px
        }, "hello, world" }
    }
}
