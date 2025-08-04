use crate::extensions::{Extension, Context};

pub struct InputExtension;

impl Extension for InputExtension {
    fn init(&mut self, ctx: &Context) {
        let ctx = ctx.clone();
        crossterm::terminal::enable_raw_mode().unwrap();
        std::thread::spawn(move || loop {
            if let Ok(e) = crossterm::event::read() {
                ctx.event(&e);
            }
        });
    }

    fn on_close(&mut self) {
        crossterm::terminal::disable_raw_mode().unwrap();
    }
}

impl crate::extensions::Event for crossterm::event::Event {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
