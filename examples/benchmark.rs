use std::{sync::mpsc, sync::Arc, thread};

use osui::prelude::*;

pub fn main() {
    let engine = Arc::new(Benchmark::new(Console::new()));

    let (tx, rx) = mpsc::channel();

    let max_threads = 32;
    let mut handles = vec![];

    for chunk in (0..500).collect::<Vec<_>>().chunks(max_threads) {
        for &items in chunk {
            let engine = engine.clone();
            let tx = tx.clone();

            let handle = thread::spawn(move || {
                let res = engine
                    .run(BenchmarkApp(items))
                    .expect("Failed to run engine");

                tx.send((items, res)).expect("Failed to send result");
            });

            handles.push(handle);
        }

        for handle in handles.drain(..) {
            handle.join().expect("Thread panicked");
        }
    }

    drop(tx);

    let mut results: Vec<(usize, BenchmarkResult)> = rx.iter().collect();
    results.sort_by_key(|(items, _)| *items);

    for (items, bench) in results {
        println!("Results for {items} items:\n{bench}");
    }
}

struct BenchmarkApp(pub usize);

impl ComponentImpl for BenchmarkApp {
    fn call(&self, cx: &Arc<Context>) -> View {
        let items = self.0.clone();
        rsx! {
            for _ in (0..items) {
                "Hello, world!"
            }
        }
        .view(cx.clone())
    }
}
