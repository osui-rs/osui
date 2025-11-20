use osui::{state::use_state, Context, Event, Node};

pub struct ClickEvent;

impl Event for ClickEvent {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

fn app(cx: &mut Context) -> Node {
    // Hooks
    let count = use_state(0);
    cx.event({
        let count = count.clone();
        move |_: &ClickEvent| {
            **count.get() += 1;
        }
    });

    Box::new(move |renderer| {
        renderer
            .draw_text(&format!("Count: {count}"), 0, 0)
            .unwrap();
    })
}

fn main() {
    let mut osui = osui::Osui::new(Box::new(app));
    loop {
        osui.render().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
