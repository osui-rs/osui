use std::{
    io::{stdout, Write},
    sync::{Arc, Mutex},
};

use crossterm::{cursor::MoveTo, execute, terminal::Clear};

use crate::component::{context::Context, ComponentImpl};
use crate::{
    engine::{commands, CommandExecutor},
    render::Area,
    DrawContext, View,
};

use super::Engine;

pub struct ConsoleExecutor {
    running: Mutex<bool>,
}

pub struct Console {
    threads: Mutex<Vec<Arc<dyn Fn(Arc<Context>) + Send + Sync>>>,
    executor: Arc<ConsoleExecutor>,
}

impl Console {
    pub fn new() -> Self {
        Self {
            threads: Mutex::new(Vec::new()),
            executor: Arc::new(ConsoleExecutor {
                running: Mutex::new(true),
            }),
        }
    }

    pub fn thread<F: Fn(Arc<Context>) + Send + Sync + 'static>(&self, run: F) {
        self.threads.lock().unwrap().push(Arc::new(run));
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

    fn render(&self, cx: &Arc<Context>) {
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
    }

    fn init<C: ComponentImpl + 'static>(&self, component: C) -> Arc<Context> {
        let cx = Context::new(component, self.executor.clone());
        cx.refresh();

        for thread in self.threads.lock().unwrap().iter() {
            let thread = thread.clone();

            std::thread::spawn({
                let cx = cx.clone();
                move || thread(cx)
            });
        }

        cx
    }

    fn executor(&self) -> Arc<dyn super::CommandExecutor> {
        self.executor.clone()
    }

    fn run<F: ComponentImpl + 'static>(&self, component: F) -> crate::Result<()> {
        let cx = self.init(component);

        while self.executor.is_running() {
            self.render(&cx);
            self.render_delay();
        }

        Ok(())
    }
}

impl ConsoleExecutor {
    pub fn is_running(self: &Arc<ConsoleExecutor>) -> bool {
        *self.running.lock().unwrap()
    }

    pub fn stop(&self) -> crate::Result<()> {
        *self.running.lock()? = false;
        Ok(())
    }
}

impl CommandExecutor for ConsoleExecutor {
    fn execute_command(&self, command: &Arc<dyn super::Command>) -> crate::Result<()> {
        let command = command.as_any();

        if let Some(commands::Stop) = command.downcast_ref() {
            self.stop()?;
        }

        Ok(())
    }
}
