use std::io::stdout;

use crossterm::{
    event::{
        DisableMouseCapture, EnableMouseCapture, KeyboardEnhancementFlags,
        PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
    },
    execute,
};

use crate::extensions::{Context, Extension};

pub struct InputExtension;

impl Extension for InputExtension {
    fn init(&mut self, ctx: &Context) {
        let ctx = ctx.clone();
        crossterm::terminal::enable_raw_mode().unwrap();
        execute!(
            stdout(),
            PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES)
        )
        .unwrap();
        execute!(stdout(), EnableMouseCapture).unwrap();
        std::thread::spawn(move || loop {
            if let Ok(e) = crossterm::event::read() {
                ctx.event(&e);
            }
        });
    }

    fn on_close(&mut self) {
        execute!(stdout(), PopKeyboardEnhancementFlags).unwrap();
        execute!(stdout(), DisableMouseCapture).unwrap();
        crossterm::terminal::disable_raw_mode().unwrap();
    }
}

impl crate::extensions::Event for crossterm::event::Event {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
