use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    state::{use_state, State},
    widget::Element,
};

pub struct Input {
    pub state: State<String>,
    cursor: usize,
}

impl Element for Input {
    fn render(
        &mut self,
        scope: &mut crate::prelude::RenderScope,
        render_context: &crate::render_scope::RenderContext,
    ) {
        let s = self.state.get();
        scope.draw_text(0, 0, &s);
        if render_context.is_focused() {
            if let Some(c) = s.chars().nth(self.cursor) {
                scope.draw_text_inverted(self.cursor as u16, 0, &c.to_string());
            } else {
                scope.draw_text_inverted(s.len() as u16, 0, " ");
            }
        }
    }

    fn event(&mut self, event: &dyn crate::prelude::Event) {
        if let Some(crossterm::event::Event::Key(KeyEvent {
            code, modifiers, ..
        })) = event.get()
        {
            if !modifiers.is_empty() && !modifiers.contains(KeyModifiers::SHIFT) {
                return;
            }

            match code {
                KeyCode::Char(c) => {
                    self.state.get().insert(self.cursor, *c);
                    self.cursor += 1;
                }
                KeyCode::Backspace => {
                    if self.cursor > 0 {
                        self.state.get().remove(self.cursor - 1);
                        self.cursor -= 1;
                    }
                }
                KeyCode::Delete => {
                    if self.state.get().len() > self.cursor {
                        self.state.get().remove(self.cursor);
                    }
                }
                KeyCode::Left => {
                    if self.cursor > 0 {
                        self.cursor -= 1;
                    }
                }
                KeyCode::Right => {
                    if self.cursor < self.state.get().len() {
                        self.cursor += 1;
                    }
                }
                _ => {}
            }
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Input {
    pub fn new() -> Self {
        Self {
            state: use_state(String::new()),
            cursor: 0,
        }
    }
}
