pub mod benchmark;
pub mod console;

pub use benchmark::*;
pub use console::*;

use std::sync::Arc;

use crate::{prelude::Context, render::Area, DrawContext, View};

pub trait Engine {
    fn run<F: Fn(&Arc<Context>) -> View + Send + Sync + 'static>(&self, component: F) {
        let cx = self.init(component);

        loop {
            self.render(&cx);
            self.render_delay();
        }
    }

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
}
