use osui::prelude::*;

pub fn main() {
    let engine = Console::new();
    engine.run(App {}).expect("Failed to run engine");
}

#[component]
fn App(cx: &Arc<Context>) -> View {
    rsx! {
        Card { content: "Hello World".to_string() }
    }
    .view(&cx)
}

#[component]
fn Card(cx: &Arc<Context>, content: String) -> View {
    let content = content.clone();

    rsx! {
        %ref content "----\n{content}\n----"
    }
    .view(&cx)
}
