use std::sync::Arc;

use osui::{prelude::*, rsx};

pub struct Count(pub State<u16>);

fn main() {
    let console = Console::new();
    console.run(app);
}

fn app(cx: &Arc<Context>) -> View {
    let count = use_state(0);
    let mount = use_mount_manual();

    // Sync state with children
    use_sync_effect(cx, &count, |v| Count(v.clone()), &[&mount, &count]);

    use_effect(
        {
            let count = count.clone();
            move || loop {
                std::thread::sleep(std::time::Duration::from_millis(500));
                let mut count = count.get();
                if *count > 5 {
                    *count = 0;
                    continue;
                }
                *count += 1;
            }
        },
        &[&mount],
    );

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

        !mount mount
    }
    .view(cx.clone())
}

fn my_component(cx: &Arc<Context>) -> View {
    // Sync state with parent
    let count = use_sync_state(cx, 0, |Count(v)| v.get_dl());

    rsx! {
        "Count: {count}"
    }
    .view(cx.clone())
}
