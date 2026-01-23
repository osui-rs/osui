use std::sync::Arc;

use osui::prelude::*;

#[derive(Debug, Clone)]
pub struct KeyPress;

impl Event for KeyPress {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

fn main() {
    let cx = Context::new(app);
    cx.refresh();

    std::thread::spawn({
        let cx = cx.clone();
        move || loop {
            crossterm::event::read().unwrap();
            cx.emit_event_threaded(&KeyPress);
        }
    });

    loop {
        cx.get_view()(&mut DrawContext {});
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}

fn app(cx: &Arc<Context>) -> View {
    let count = use_state(0);

    cx.on_event({
        let count = count.clone();
        move |_cx, _event: &KeyPress| {
            #[allow(unused_mut)]
            let mut count = count.get();
            *count += 1;
        }
    });

    Arc::new(move |_| {
        println!("Count: {count}");
    })
}
