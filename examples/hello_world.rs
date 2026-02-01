use osui::prelude::*;

pub fn main() {
    let engine = Console::new();
    engine.run(App {}).expect("Failed to run engine");
}

#[component]
fn App(cx: &Arc<Context>) -> View {
    rsx! {
        impl my_center(11, 1)
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

fn my_center(ctx: &mut DrawContext, view: View, w: u16, h: u16) {
    let (x, y) = ((ctx.area.width - w) / 2, (ctx.area.height - h) / 2);
    let area = ctx.allocate(x, y, w, h);
    ctx.draw_view(area, view);
}
