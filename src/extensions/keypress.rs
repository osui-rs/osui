use crossterm::event::KeyEvent;

use crate::{
    events::{Close, Event, EventManager},
    extensions::Extension,
    Screen,
};

impl Event for KeyEvent {
    fn as_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}

pub trait KeyPressEventHandler {
    fn on_keypress(&mut self, events: &mut EventManager, event: Box<KeyEvent>);
}

pub struct KeyPressExtension;

impl Extension for KeyPressExtension {
    fn init(&mut self, _: &mut Screen, events: &mut EventManager) {
        crossterm::terminal::enable_raw_mode().unwrap();
        crate::utils::clear().unwrap();
        crate::utils::hide_cursor().unwrap();

        events.on(|_, _: Box<Close>| {
            crossterm::terminal::disable_raw_mode().unwrap();
            crate::utils::show_cursor().unwrap();
            crate::utils::clear().unwrap();
        });
    }

    fn tick_start(&mut self, _: &mut Screen, _: &mut EventManager) {}

    fn tick_end(&mut self, _: &mut Screen, events: &mut EventManager) {
        if crossterm::event::poll(std::time::Duration::from_millis(13)).unwrap() {
            match crossterm::event::read().unwrap() {
                crossterm::event::Event::Key(e) => {
                    events.dispatch(e);
                }
                _ => {}
            }
        }
    }
}
