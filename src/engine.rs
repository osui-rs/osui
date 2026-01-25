use std::{
    io::{stdout, Write},
    sync::{Arc, Mutex},
};

use crossterm::{cursor::MoveTo, execute, terminal::Clear};

use crate::{prelude::Context, render::Area, DrawContext, View};

pub trait Engine {
    fn render_view(&self, area: &Area, view: &View) -> DrawContext;
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

            execute!(stdout(), Clear(crossterm::terminal::ClearType::Purge)).unwrap();
            execute!(stdout(), Clear(crossterm::terminal::ClearType::All)).unwrap();

            self.draw_context(&self.render_view(
                &Area {
                    x: 0,
                    y: 0,
                    width,
                    height,
                },
                &cx.get_view(),
            ));

            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    }
}

impl Engine for Console {
    fn render_view(&self, area: &Area, view: &View) -> DrawContext {
        let mut context = DrawContext::new(area.clone());
        view(&mut context);
        context
    }

    fn draw_context(&self, ctx: &DrawContext) {
        for inst in &ctx.drawing {
            match inst {
                crate::render::DrawInstruction::Text(point, text) => {
                    let (x, y) = (ctx.area.x + point.x, ctx.area.y + point.y);
                    execute!(stdout(), MoveTo(x, y),).unwrap();
                    print!("{text}");
                    stdout().flush().unwrap();
                }
                crate::render::DrawInstruction::Child(_point, child) => self.draw_context(child),
                crate::render::DrawInstruction::View(area, view) => {
                    self.draw_context(&self.render_view(area, view))
                }
            }
        }
    }
}
