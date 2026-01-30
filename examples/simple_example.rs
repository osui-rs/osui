use std::sync::Arc;

use osui::prelude::*;

pub fn main() {
    let engine = Console::new();
    engine.run(App {}).expect("Failed to run engine");
}

#[component]
fn App(cx: &Arc<Context>) -> View {
    rsx! {
        MyComponent { "Hello world!" }
    }
    .view(&cx)
}

#[component]
fn MyComponent(cx: &Arc<Context>, children: &Rsx) -> View {
    children.view(&cx)
}
