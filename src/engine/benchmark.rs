//! # Benchmark Module
//!
//! Provides performance benchmarking capabilities for rendering engines.

use std::{io::stdout, sync::Arc, time::Instant};

use crossterm::{cursor::MoveTo, execute, terminal::Clear};

use crate::component::{context::Context, ComponentImpl};
use crate::{render::Area, DrawContext, View};

use super::Engine;

/// Results of a benchmark run
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    /// Average render time in microseconds
    pub average: u128,
    /// Minimum render time in microseconds
    pub min: u128,
    /// Maximum render time in microseconds
    pub max: u128,
    /// Total time spent rendering in microseconds
    pub total_render: u128,
    /// Total benchmark time including setup in microseconds
    pub total: u128,
}

/// Wraps an engine to benchmark its rendering performance
pub struct Benchmark<T: Engine>(T);

impl<T: Engine> Benchmark<T> {
    /// Creates a new benchmark wrapper around the given engine
    pub fn new(engine: T) -> Self {
        Self(engine)
    }
}

impl<T: Engine> Engine<BenchmarkResult> for Benchmark<T> {
    fn run<F: ComponentImpl + 'static>(&self, component: F) -> crate::Result<BenchmarkResult> {
        let mut times: Vec<u128> = Vec::new();
        let cx = self.init(component);

        let start = Instant::now();

        // Run 40 render cycles and measure each
        for _ in 0..40 {
            let start = Instant::now();
            self.render(&cx);
            let end = Instant::now();
            times.push(end.duration_since(start).as_micros());
        }

        let end = Instant::now();

        execute!(stdout(), Clear(crossterm::terminal::ClearType::Purge)).unwrap();
        execute!(stdout(), Clear(crossterm::terminal::ClearType::All)).unwrap();
        execute!(stdout(), MoveTo(0, 0)).unwrap();

        Ok(BenchmarkResult {
            min: *times.iter().min().unwrap_or(&0),
            max: *times.iter().max().unwrap_or(&0),
            total: end.duration_since(start).as_micros(),
            total_render: times.iter().sum::<u128>(),
            average: if times.len() > 0 {
                times.iter().sum::<u128>() / (times.len() as u128)
            } else {
                0
            },
        })
    }

    fn init<F: ComponentImpl + 'static>(&self, component: F) -> Arc<Context> {
        self.0.init(component)
    }

    fn draw_context(&self, ctx: &DrawContext) {
        self.0.draw_context(ctx)
    }

    fn render(&self, cx: &Arc<Context>) {
        self.0.render(cx)
    }

    fn render_view(&self, area: &Area, view: &View) -> DrawContext {
        self.0.render_view(area, view)
    }

    fn render_delay(&self) {
        self.0.render_delay();
    }

    fn executor(&self) -> Arc<dyn super::CommandExecutor> {
        self.0.executor()
    }
}

impl std::fmt::Display for BenchmarkResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "- Average: {} µs\n\
             - Min: {} µs\n\
             - Max: {} µs\n\
             - Total Render: {} µs\n\
             - Total: {} µs",
            self.average, self.min, self.max, self.total_render, self.total
        )
    }
}
