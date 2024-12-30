use osui::prelude::*;

fn main() {
    launch!(App);
}

#[component]
pub fn App() {
    rsx! {
        text { style: style!{
            outline: true,
        }, "Hello, World!\nabc" }

        text { style: style!{
            outline: true,
            width: 3 px
        }, "Hello, World!\nabc testing" }
    }
}
