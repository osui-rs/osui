use std::sync::Arc;

use osui::prelude::*;

fn main() {
    let mut cx = Context::new(app);
    cx.refresh();
    // println!("{:?}", cx.get_nodes());
}

fn app(cx: &mut Context) -> Vec<Node> {
    let count = use_state(0);

    vec![Node::String(Arc::new(move || format!("{count}")))]
}
