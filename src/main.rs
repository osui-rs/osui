use osui::prelude::*;

fn main() {
    launch!(App);
}

#[component]
fn App() {
    let count = use_state(20);

    rsx! {
        %text { "Hello" }
        button { on_click: fn (_, _, _) @count {
            count += 1
        }, "World: {count}" }
    }
}
