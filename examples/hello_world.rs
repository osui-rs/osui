use std::sync::Arc;

use osui::prelude::*;

pub fn main() {
    let engine = Console::new();
    engine.run(App {}).expect("Failed to run engine");
}

#[component]
fn App(cx: &Arc<Context>) -> View {
    rsx! {
        "Hello World"
    }
    .view(&cx)
}
