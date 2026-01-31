use osui::prelude::*;

use std::collections::HashMap;

pub fn main() {
    let engine = Arc::new(Benchmark::new(Console::new()));

    let mut benchmark_result: HashMap<(usize, usize), BenchmarkResult> = HashMap::new();

    for i in 0..15 {
        for n in 0..15 {
            let res = {
                let mut results = Vec::with_capacity(6);

                for _ in 0..6 {
                    results.push(
                        engine
                            .run(App {
                                n: n * 72,
                                i: i * 72,
                            })
                            .expect("Failed to run engine"),
                    );
                }

                results.sort_by_key(|r| r.total_render);
                results[3].clone()
            };

            benchmark_result.insert((i, n as usize), res);
        }
    }

    let max = benchmark_result
        .values()
        .map(|b| b.total_render)
        .max()
        .unwrap_or(0);

    println!("Iterx72,Nestingx72,Time µs\n14,14,{}", { max + 500 });

    for ((i, n), bench) in benchmark_result.iter() {
        println!("{i},{n},{}", bench.total_render);
    }
}

#[component]
fn App(cx: &Arc<Context>, n: usize, i: usize) -> View {
    let n = n.clone();
    let i = i.clone();

    if n == 0 {
        rsx! {
            "Hello, world!"
        }
        .view(&cx)
    } else {
        rsx! {
            for _ in (0..i) {
                    App { n: n - 1, i: 0 }
            }
        }
        .view(&cx)
    }
}
