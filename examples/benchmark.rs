use std::sync::Arc;

use osui::prelude::*;

pub fn main() {
    let engine = Benchmark::new(Console::new());
    let mut benchmark_result = Vec::new();

    for items in 0..10 {
        benchmark_result.push(
            engine
                .run(move |cx| {
                    rsx! {
                        for _ in (0..items) {
                            "Hello, world!"
                        }
                    }
                    .view(cx.clone())
                })
                .expect("Failed to run engine"),
        );
    }

    println!("{benchmark_result:?}");
}
