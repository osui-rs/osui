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
        move |_, _: &KeyPress| {
            let mut count = count.get();

            if *count > 5 {
                *count = 0;

                return;
            }

            *count += 1;
        }
    });

    {
        let scope = cx.scope();

        scope.child(
            my_component,
            Some(Arc::new(|ctx, view| {
                let area = ctx.allocate(0, 0, 0, 0);
                ctx.draw_view(area, view)
            })),
        );
    }

    {
        cx.dyn_scope(
            {
                let count = count.clone();
                move |scope| {
                    if *count.get() != 0 {
                        if scope.children.lock().unwrap().len() == 0 {
                            scope.child(
                                my_component,
                                Some(Arc::new(|ctx, view| {
                                    let area = ctx.allocate(0, 2, 0, 0);
                                    ctx.draw_view(area, view)
                                })),
                            );
                        }
                    } else {
                        scope.children.lock().unwrap().clear();
                    }
                }
            },
            &[&count],
        );
    }

    Arc::new({
        let cx = cx.clone();

        move |ctx| {
            cx.draw_children(ctx);
        }
    })
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

    Arc::new(move |ctx| {
        ctx.draw_text(Point { x: 0, y: 0 }, &format!("Count: {count}"));
    })
}
