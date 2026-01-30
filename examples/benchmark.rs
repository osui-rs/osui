use std::sync::Arc;

use osui::prelude::*;

pub fn main() {
    let engine = Benchmark::new(Console::new());
    engine.run(app).expect("Failed to run engine");
}

fn app(cx: &Arc<Context>) -> View {
    rsx! {
        "Hello, world!"
    }
    .view(cx.clone())
}
