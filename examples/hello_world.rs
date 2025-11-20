use osui::{Context, Node};

fn app(cx: &mut Context) -> Node {
    // Hooks
    // Event handlers
    // cx.event(|e: &ClickEvent| {
        
    // });

    Box::new(|renderer| {
        renderer.draw_text("Hello, World!", 0, 0).unwrap();
    })
}

fn main() {
    let mut osui = osui::Osui::new(Box::new(app));
    loop {
        osui.render().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
