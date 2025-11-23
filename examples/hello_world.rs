use std::sync::Arc;

use osui::prelude::*;

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
            cx.event(&KeyPress);
        }
    });

    loop {
        println!("{:?}", cx.get_nodes());
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}

fn app(cx: &Arc<Context>) -> Vec<Node> {
    let count = use_state(0);
    let count2 = use_state(0);

    use_effect(
        {
            let count2 = count2.clone();
            move || {
                *count2.get() += 1;
            }
        },
        &[&count],
    );

    cx.on_event({
        let count = count.clone();
        move |_cx, _event: &KeyPress| {
            #[allow(unused_mut)]
            let mut count = count.get();
            *count += 1;
        }
    });

    vec![Node::String(Arc::new(move || format!("{count} {count2}")))]
}
