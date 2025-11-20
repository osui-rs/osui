use osui::{Context, Event, Node};

pub struct ClickEvent;

impl Event for ClickEvent {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

fn app(cx: &mut Context) -> Node {
    // Hooks
    cx.event(|_: &ClickEvent| {});

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
