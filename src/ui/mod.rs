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
pub struct Text {
    pub on_event: Handler<Text<'a>>,
}

impl ElementWidget for Text<'_> {
    fn render(&self, writer: &mut crate::Writer) {
        if let Children::Text(text) = &self.children {
            writer.write(&text);
        }
    }
    fn event(&mut self, event: Event, document: &Document) {
        call!(self.on_event(event, document));
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
        let (width, height) = writer.get_size();
        let mut frame = crate::utils::Frame::new(width, height);

        if let Children::Children(children, child) = &self.children {
            for (i, elem) in children.iter().enumerate() {
                frame.render(writer.get_focused() && i == *child, elem);
            }
        }

        writer.write(&frame.output());
    }

    fn event(&mut self, event: Event, document: &Document) {
        if event == Event::FocusGained {
            if let Some(styling) = self.styling.clone() {
                self.set_styling(&styling);
            }
        }
        if let Some(child) = self.children.get_child() {
            child.event(event, document);
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
                    sleep(100);
                    self.style.set_state("");
                }
            }
            _ => {}
        }
    }
}

#[element]
#[derive(Debug)]
pub struct DataHolder<'a, T> {
    pub data: T,
}

pub fn data_holder<'a, T: std::default::Default>() -> Box<DataHolder<'a, T>> {
    Box::new(DataHolder {
        style: Style::default(),
        data: T::default(),
        children: Children::None,
        class: "",
        id: "",
    })
}

impl<'a, T: std::fmt::Debug + Send + Sync> ElementWidget for DataHolder<'a, T> {
    fn render(&self, _writer: &mut crate::Writer) {}
}
