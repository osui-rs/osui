use std::sync::{Arc, Mutex};

use crate::{prelude::Context, DrawContext, View};

pub trait Engine {
    fn render(&self, ctx: &mut DrawContext, view: View);
    fn draw_text(&self, ctx: &mut DrawContext, text: &str);
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

        let (width, height) = crossterm::terminal::size().unwrap();

        loop {
            cx.get_view()(&mut DrawContext::new(width, height));
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    }
}

impl Engine for Console {
    fn render(&self, ctx: &mut DrawContext, view: View) {
        view(&mut DrawContext::new(
            ctx.available.width,
            ctx.available.height,
        ));
    }

    fn draw_text(&self, _ctx: &mut DrawContext, text: &str) {
        println!("{}", text);
    }
}
