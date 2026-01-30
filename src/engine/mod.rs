pub mod benchmark;
pub mod commands;
pub mod console;
pub mod simple_benchmark;

pub use benchmark::*;
pub use console::*;
pub use simple_benchmark::*;

use std::{any::Any, sync::Arc};

use crate::{prelude::Context, render::Area, DrawContext, View};

pub trait Engine<Output = ()> {
    fn run<F: Fn(&Arc<Context>) -> View + Send + Sync + 'static>(
        &self,
        component: F,
    ) -> crate::Result<Output>;

    fn init<F: Fn(&Arc<Context>) -> View + Send + Sync + 'static>(
        &self,
        component: F,
    ) -> Arc<Context>;
    fn render(&self, cx: &Arc<Context>);
    fn render_delay(&self) {
        std::thread::sleep(std::time::Duration::from_millis(16))
    }

    fn render_view(&self, area: &Area, view: &View) -> DrawContext;
    fn draw_context(&self, ctx: &DrawContext);
    fn executor(&self) -> Arc<dyn CommandExecutor>;
}

pub trait Command {
    fn as_any(&self) -> &dyn Any;
}

pub trait CommandExecutor: Send + Sync {
    fn execute_command(&self, command: &Arc<dyn Command>) -> crate::Result<()>;
}
