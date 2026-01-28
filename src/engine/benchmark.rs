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
        let mut times: Vec<usize> = Vec::new();
        let cx = self.init(component);

        let start = Instant::now();

        for _ in 0..100 {
            let start = Instant::now();
            self.render(&cx);
            let end = Instant::now();
            times.push(end.duration_since(start).as_millis() as usize);
            self.0.render_delay();
        }

        let end = Instant::now();

        execute!(stdout(), Clear(crossterm::terminal::ClearType::Purge)).unwrap();
        execute!(stdout(), Clear(crossterm::terminal::ClearType::All)).unwrap();
        execute!(stdout(), MoveTo(0, 0)).unwrap();

        println!(
            "Average time: {}ms",
            if times.len() > 0 {
                times.iter().sum::<usize>() / times.len()
            } else {
                0
            }
        );
        println!("Min time: {}ms", times.iter().min().unwrap_or(&0));
        println!("Max time: {}ms", times.iter().max().unwrap_or(&0));
        println!(
            "Median time: {}ms",
            times.iter().nth(times.len() / 2).unwrap_or(&0)
        );
        println!(
            "90th percentile: {}ms",
            times.iter().nth(times.len() * 90 / 100).unwrap_or(&0)
        );
        println!(
            "95th percentile: {}ms",
            times.iter().nth(times.len() * 95 / 100).unwrap_or(&0)
        );
        println!(
            "99th percentile: {}ms",
            times.iter().nth(times.len() * 99 / 100).unwrap_or(&0)
        );

        println!("Total time: {}ms", end.duration_since(start).as_millis());
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
