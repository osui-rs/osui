use std::sync::Arc;

use crossterm::event::KeyEvent;

use crate::{
    event, event_handler, events::EventManager, extensions::keypress::KeyPressEventHandler, Element,
};

pub struct Input(String, usize);

impl Input {
    pub fn new() -> Input {
        Input(String::new(), 0)
    }
}

impl Element for Input {
    fn init(&mut self, events: &Arc<EventManager>) {
        event_handler!(Self, self, events, on_keypress);
    }

    fn render(&mut self, scope: &mut crate::render_scope::RenderScope) {
        let mut s = self.0.clone();

        if self.1 < self.0.len() {
            s.insert_str(self.1 + 1, "\x1b[0m");
            s.insert_str(self.1, "\x1b[42m");
        } else {
            s += "\x1b[42m \x1b[0m";
        }

        scope.draw_text(&s);
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl KeyPressEventHandler for Input {
    fn on_keypress(&mut self, events: &Arc<EventManager>, event: &KeyEvent) {
        match event.code {
            crossterm::event::KeyCode::Char(c) => {
                self.0.insert(self.1, c);
                self.1 += 1;
                events.dispatch(InputUpdateEvent(self.0.clone()));
            }
            crossterm::event::KeyCode::Backspace => {
                if self.1 > 0 {
                    self.0.remove(self.1 - 1);
                    self.1 -= 1;
                    events.dispatch(InputUpdateEvent(self.0.clone()));
                }
            }
            crossterm::event::KeyCode::Left => {
                if self.1 > 0 {
                    self.1 -= 1;
                    events.dispatch(InputUpdateEvent(self.0.clone()));
                }
            }
            crossterm::event::KeyCode::Right => {
                if self.1 < self.0.len() {
                    self.1 += 1;
                    events.dispatch(InputUpdateEvent(self.0.clone()));
                }
            }
            _ => {
                events.dispatch(InputKeyPress(self.0.clone(), *event));
            }
        }
    }
}

event!(InputUpdateEvent(pub String));
event!(InputKeyPress(pub String, pub KeyEvent));
