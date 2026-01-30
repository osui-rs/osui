use std::sync::Arc;

use osui::prelude::*;

pub fn main() {
    let engine = Console::new();
    engine.run(App {}).expect("Failed to run engine");
}

#[component]
fn App(cx: &Arc<Context>) -> View {
    rsx! {
        MyComponent { "------ example" }
    }
    .view(&cx)
}

#[component]
fn MyComponent(cx: &Arc<Context>, children: &Rsx) -> View {
    rsx! {
        @{children}
        "Simple"
    }
    .view(&cx)
}
