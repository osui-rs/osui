use std::{io::stdout, sync::Arc, time::Instant};

use crossterm::{cursor::MoveTo, execute, terminal::Clear};

use crate::{prelude::Context, render::Area, DrawContext, View};

use super::Engine;

pub struct Benchmark<T: Engine>(T);

impl<T: Engine> Benchmark<T> {
    pub fn new(engine: T) -> Self {
        Self(engine)
    }
}

impl<T: Engine> Engine for Benchmark<T> {
    fn run<F: Fn(&Arc<Context>) -> View + Send + Sync + 'static>(&self, component: F) {
        let mut times: Vec<u64> = Vec::new();
        let cx = self.init(component);

        let start = Instant::now();

        for _ in 0..100 {
            let start = Instant::now();
            self.render(&cx);
            let end = Instant::now();
            times.push(end.duration_since(start).as_nanos() as u64);
            self.0.render_delay();
        }

        let end = Instant::now();

        execute!(stdout(), Clear(crossterm::terminal::ClearType::Purge)).unwrap();
        execute!(stdout(), Clear(crossterm::terminal::ClearType::All)).unwrap();
        execute!(stdout(), MoveTo(0, 0)).unwrap();

        println!(
            "Average time: {} nanoseconds",
            if times.len() > 0 {
                times.iter().sum::<u64>() / (times.len() as u64)
            } else {
                0
            }
        );
        println!("Min time: {} nanoseconds", times.iter().min().unwrap_or(&0));
        println!("Max time: {} nanoseconds", times.iter().max().unwrap_or(&0));
        println!(
            "Median time: {} nanoseconds",
            times.iter().nth(times.len() / 2).unwrap_or(&0)
        );
        println!(
            "Total time: {} nanoseconds",
            end.duration_since(start).as_nanos()
        );
        println!(
            "Total render time: {} nanoseconds",
            times.iter().sum::<u64>()
        );
    }

    fn init<F: Fn(&Arc<Context>) -> View + Send + Sync + 'static>(
        &self,
        component: F,
    ) -> Arc<Context> {
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
