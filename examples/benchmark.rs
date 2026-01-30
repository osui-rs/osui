use std::sync::Arc;

use osui::prelude::*;

pub fn main() {
    let engine = Benchmark::new(Console::new());

    let mut benchmark_result = Vec::new();

    for flat in 0..10 {
        // for nest in 0..10 {
        benchmark_result.push(
            engine
                .run(move |cx| {
                    // let sync_nest = use_state(nest);
                    cx.scope().child(app, None);
                    // use_sync_effect(cx, &sync_nest, |s| s.get_dl(), &[&sync_nest]);

                    rsx! {
                        app
                    }
                    .view(cx.clone())
                })
                .expect("Failed to run engine"),
        );
        // }
    }
}

fn app(cx: &Arc<Context>) -> View {
    // let nest = use_sync_state(cx, 0, |v| *v);

    rsx! {
        "hello_world"
    }
    .view(cx.clone())
}
