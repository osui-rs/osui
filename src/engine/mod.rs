pub mod benchmark;
pub mod commands;
pub mod console;

pub use benchmark::*;
pub use console::*;

use std::{any::Any, sync::Arc};

use crate::component::{context::Context, ComponentImpl};
use crate::{render::Area, DrawContext, View};

pub trait Engine<Output = ()> {
    fn run<C: ComponentImpl + 'static>(&self, component: C) -> crate::Result<Output>;
    fn init<C: ComponentImpl + 'static>(&self, component: C) -> Arc<Context>;
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
