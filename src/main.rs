use osui::prelude::*;

fn main() {
    launch!(App);
}

#[component]
pub fn App() {
    rsx! {
        @SetStyle(css! {
            div {
                outline: true,
                width: 33%,
                x: Auto,
                y: 0 px,
            }
        })

        div { class: "div" }
        div { class: "div" }
        div { class: "div" }
    }
}
