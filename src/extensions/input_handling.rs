use crate::{extensions::Extension, prelude::Handler};

pub struct InputExtension;

impl Extension for InputExtension {
    fn init(&mut self, screen: std::sync::Arc<crate::Screen>) {
        crossterm::terminal::enable_raw_mode().unwrap();
        std::thread::spawn(move || loop {
            if let Ok(e) = crossterm::event::read() {
                for widget in screen.widgets.lock().unwrap().iter() {
                    if let Some(h) = widget.get::<Handler<crossterm::event::Event>>() {
                        h.call(widget, &e);
                    }
                }
            }
        });
    }

    fn on_close(&mut self, _screen: std::sync::Arc<crate::Screen>) {
        crossterm::terminal::disable_raw_mode().unwrap();
    }
}

impl crate::extensions::Event for crossterm::event::Event {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
