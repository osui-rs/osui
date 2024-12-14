/// The `ui` module defines the basic building blocks for creating a terminal-based UI framework.
///
/// # Modules
/// - `styles`: A module for defining UI styles.
pub mod styles;

pub use styles::*;

use crate::prelude::*;
use osui_element::{elem_fn, element};

#[element]
#[elem_fn]
#[derive(Default, Debug)]
pub struct Text {}

impl ElementWidget for Text<'_> {
    fn render(&self, writer: &mut crate::Writer) {
        writer.write(&self.children.get_text());
    }
}

#[element]
#[elem_fn]
#[derive(Default, Debug)]
pub struct Div {
    pub styling: Option<std::collections::HashMap<StyleName, Style>>,
}

impl ElementWidget for Div<'_> {
    fn render(&self, writer: &mut crate::Writer) {
        let mut frame = writer.new_frame();

        if let Children::Children(children, child) = &self.children {
            for (i, elem) in children.iter().enumerate() {
                frame.render(writer.get_focused() && i == *child, elem);
            }
        }
    }

    fn event(&mut self, event: Event, document: &Document) {
        if event == Event::FocusGained {
            if let Some(styling) = self.styling.clone() {
                self.set_styling(&styling);
            }
        }
        if let (Some((child, i)), l) = self.children.get_child_idx() {
            match event {
                Event::Key(k) => match k.code {
                    KeyCode::Tab => {
                        child.event(Event::FocusLost, document);
                        if *i < l - 1 {
                            *i += 1;
                        } else {
                            *i = 0;
                        }
                        child.event(Event::FocusGained, document);
                    }
                    KeyCode::BackTab => {
                        child.event(Event::FocusLost, document);
                        if *i > 0 {
                            *i -= 1;
                        } else {
                            *i = l - 1;
                        }
                        child.event(Event::FocusGained, document);
                    }
                    _ => child.event(event, document),
                },
                _ => child.event(event, document),
            }
        }
    }
}

#[element]
#[elem_fn]
#[derive(Default, Debug)]
pub struct Button {
    pub on_click: Handler<Button<'a>>,
}

impl ElementWidget for Button<'_> {
    fn render(&self, writer: &mut crate::Writer) {
        writer.write(&self.children.get_text());
    }

    fn event(&mut self, event: Event, document: &Document) {
        match event {
            Event::Key(key) => {
                if key.code == KeyCode::Enter {
                    document.render();
                    self.style.set_state("clicked");
                    call!(self.on_click(event, document));
                    document.render();
                    sleep(65);
                    self.style.set_state("");
                }
            }
            _ => {}
        }
    }
}
