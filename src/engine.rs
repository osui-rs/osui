use std::sync::{Arc, Mutex};

use crate::{prelude::Context, DrawContext, View};

pub trait Engine {
    fn render_view(&self, ctx: &DrawContext, view: View);
    fn draw_context(&self, ctx: &DrawContext);
}

pub struct Console {
    threads: Mutex<Vec<Arc<dyn Fn(Arc<Context>) + Send + Sync>>>,
}

impl Console {
    pub fn new() -> Self {
        Self {
            threads: Mutex::new(Vec::new()),
        }
    }

    pub fn thread<F: Fn(Arc<Context>) + Send + Sync + 'static>(&self, run: F) {
        self.threads.lock().unwrap().push(Arc::new(run));
    }

    pub fn run<F: Fn(&Arc<Context>) -> View + Send + Sync + 'static>(&self, component: F) {
        let cx = Context::new(component);
        cx.refresh();

        for thread in self.threads.lock().unwrap().iter() {
            let thread = thread.clone();

            std::thread::spawn({
                let cx = cx.clone();
                move || thread(cx)
            });
        }

        loop {
            let (width, height) = crossterm::terminal::size().unwrap();
            let mut ctx = DrawContext::new(width, height);
            self.render_view(&mut ctx, cx.get_view());
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    }
}

impl Engine for Console {
    fn render_view(&self, ctx: &DrawContext, view: View) {
        let mut context = DrawContext::new(ctx.allocated.width, ctx.allocated.height);
        view(&mut context);
        self.draw_context(&context);
    }

    fn draw_context(&self, ctx: &DrawContext) {
        for inst in &ctx.drawing {
            match inst {
                crate::render::DrawInstruction::Text(point, text) => println!("{text}"),
                crate::render::DrawInstruction::Child(point, child) => self.draw_context(child),
            }
        }
    }
}
