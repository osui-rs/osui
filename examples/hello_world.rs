use std::sync::Arc;

use osui::prelude::*;

pub fn main() {
    let engine = Console::new();
    engine.run(App).expect("Failed to run engine");
}

struct App;

impl ComponentImpl for App {
    fn call(&self, cx: &Arc<Context>) -> View {
        rsx! {
            "Hello, world!"
        }
        .view(cx.clone())
    }
}
