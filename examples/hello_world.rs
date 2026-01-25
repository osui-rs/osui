use std::sync::Arc;

use osui::{prelude::*, rsx};

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
        move |_, _: &KeyPress| {
            let mut count = count.get();

            if *count > 5 {
                *count = 0;
                return;
            }

            *count += 1;
        }
    });

    rsx! {
        // Static scope
        my_component (ctx, view) {
            let area = ctx.allocate(5, 0, 10, 10);
            ctx.draw_view(area, view);
        }

        // Dynamic scope
        if %count (*count.get() > 5) {
            my_component (ctx, view) {
                let area = ctx.allocate(18, 0, 10, 10);
                ctx.draw_view(area, view);
            }
        }

        // Dynamic scope
        for %count (i in 0..count.get_dl()) {
            "{i}" @Point { x: 0, y: i };
        }
    }
    .view(cx.clone())
}

fn my_component(cx: &Arc<Context>) -> View {
    let count = use_state(1);

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

    rsx! {
        "Count: {count}"
    }
    .view(cx.clone())
}
