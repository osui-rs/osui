use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

use osui::prelude::*;

pub fn main() {
    let engine = Arc::new(Benchmark::new(Console::new()));
    let benchmark_result = Arc::new(Mutex::new(HashMap::new()));
    let benchmark_running = Arc::new(Mutex::new(HashSet::new()));

    for items in 0..1000 {
        let engine = engine.clone();
        let benchmark_result = benchmark_result.clone();
        let benchmark_running = benchmark_running.clone();

        std::thread::spawn(move || {
            benchmark_running.lock().unwrap().insert(items);

            let res = engine
                .run(move |cx| {
                    rsx! {
                        for _ in (0..items) {
                            "Hello, world!"
                        }
                    }
                    .view(cx.clone())
                })
                .expect("Failed to run engine");

            benchmark_result.lock().unwrap().insert(items, res);
            benchmark_running.lock().unwrap().remove(&items);
        });
    }

    while !benchmark_running.lock().unwrap().is_empty() {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    let r = benchmark_result.lock().unwrap().clone();
    let mut bench_results = r.iter().collect::<Vec<_>>();

    bench_results.sort_by_key(|(_, bench)| bench.total_render);

    println!(
        "{}",
        bench_results
            .iter()
            .map(|(key, bench)| format!("Results for {key} items:\n{bench}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}
