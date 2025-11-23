use std::sync::Arc;

use osui::prelude::*;

fn main() {
    let mut cx = Context::new(app);
    cx.refresh();

    std::thread::spawn({
        let cx = cx.clone();
        move || {}
    });

    loop {
        println!("{:?}", cx.get_nodes());
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}

fn app(cx: &Arc<Context>) -> Vec<Node> {
    let count = use_state(0);

    vec![Node::String(Arc::new(move || format!("{count}")))]
}
