use osui::prelude::*;

pub fn main() {
    let engine = Console::new();
    engine.run(App {}).expect("Failed to run engine");
}

#[component]
fn App(cx: &Arc<Context>) -> View {
    rsx! {
        // redraw is important because the effects won't be applied
        impl size_auto, center, redraw
        Card { content: "Hello World".to_string() }
    }
    .view(&cx)
}

#[component]
fn Card(cx: &Arc<Context>, content: String) -> View {
    let bar = "-".repeat(content.len());

    rsx! {
        "{bar}\n{content}\n{bar}"
    }
    .view(&cx)
}
