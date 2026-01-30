use std::sync::Arc;

use osui::prelude::*;

pub fn main() {
    let engine = Benchmark::new(Console::new());
    let benchmark_result = engine.run(App {}).expect("Failed to run engine");
    println!("Avg: {} μs", benchmark_result.average);
    println!("Min: {} μs", benchmark_result.min);
    println!("Max: {} μs", benchmark_result.max);
    println!("Tot: {} μs", benchmark_result.total);
    println!("Tot Render: {} μs", benchmark_result.total_render);
}

#[component]
fn App(cx: &Arc<Context>) -> View {
    rsx! {
        "Hello, world!"
    }
    .view(&cx)
}
