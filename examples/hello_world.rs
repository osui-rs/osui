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
    let console = Console::new();

    console.thread(move |ctx| loop {
        crossterm::event::read().unwrap();
        ctx.emit_event_threaded(&KeyPress);
    });

    console.run(app);
}

fn app(cx: &Arc<Context>) -> View {
    let count = use_state(0);

    cx.on_event({
        let count = count.clone();
        move |_cx, _event: &KeyPress| {
            let mut count = count.get();
            if *count > 5 {
                _cx.refresh();
                return;
            }
            *count += 1;
        }
    });

    Arc::new(move |ctx| {
        ctx.draw_text(Point { x: 0, y: 0 }, &format!("Count: {count}"));
        let area = ctx.allocate(0, 0, 10, 10);
        ctx.draw_view(area, Arc::new(my_view));
    })
}

fn my_view(ctx: &mut DrawContext) {
    let count = 67;

    ctx.draw_text(Point { x: 0, y: 0 }, &format!("Count: {count}"));
}
