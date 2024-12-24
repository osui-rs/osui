/// The `ui` module defines the basic building blocks for creating a terminal-based UI framework.
///
/// # Modules
/// - `styles`: A module for defining UI styles.
pub mod styles;

pub use styles::*;

use crate::prelude::*;
use osui_element::element;

#[element]
#[derive(Default, Debug)]
pub struct Text {}

impl ElementWidget for Text<'_> {
    fn render(&self, writer: &mut crate::Writer) {
        writer.write(&self.children.get_text());
    }
}

#[component]
pub fn text<'a>(class: &'a str, id: &'a str, children: Children, style: Style) -> Box<Text<'a>> {
    Box::new(Text {
        class: self.class,
        id: self.id,
        children: self.children,
        style: self.style,
    })
}

#[derive(Debug)]
pub enum Instruction {
    SetStyle(Css),
}

#[element]
#[derive(Default, Debug)]
pub struct Div {
    pub instructions: Vec<Instruction>,
}

impl ElementWidget for Div<'_> {
    fn render(&self, writer: &mut crate::Writer) {
        let focused = writer.get_focused();
        let mut frame = writer.new_frame();

        if let Children::Children(children, child) = &self.children {
            for (i, elem) in children.iter().enumerate() {
                frame.render(focused && i == *child, elem);
            }
        }
    }

    fn event(&mut self, event: Event, document: &mut Document) {
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

    fn initialize(&mut self, document: &mut Document) {
        for inst in &self.instructions {
            match inst {
                Instruction::SetStyle(s) => {
                    document.css.extend(s.clone().into_iter());
                }
            }
        }
    }
}

#[component]
pub fn div<'a>(
    class: &'a str,
    id: &'a str,
    children: Children,
    style: Style,
    instructions: Vec<Instruction>,
) -> Box<Div<'a>> {
    Box::new(Div {
        children: self.children,
        class: self.class,
        id: self.id,
        style: self.style,
        instructions: self.instructions,
    })
}

#[element]
#[derive(Default, Debug)]
pub struct Button {
    pub on_click: Handler<Button<'a>>,
    pub on_hover: Handler<Button<'a>>,
}

impl ElementWidget for Button<'_> {
    fn render(&self, writer: &mut crate::Writer) {
        writer.write(&self.children.get_text());
    }

    fn event(&mut self, event: Event, document: &mut Document) {
        match event {
            Event::Key(key) => {
                if key.code == KeyCode::Enter {
                    document.render();
                    self.style.set_state("clicked");
                    self.on_click.clone().call(self, event, document);
                    document.render();
                    sleep(65);
                    self.style.set_state("");
                }
            }
            Event::FocusGained => {
                self.on_hover.clone().call(self, event, document);
            }
            _ => {}
        }
    }
}

#[component]
pub fn button<'a>(
    class: &'a str,
    id: &'a str,
    children: Children,
    style: Style,
    on_click: Handler<Button<'a>>,
    on_hover: Handler<Button<'a>>,
) -> Box<Button<'a>> {
    Box::new(Button {
        class: self.class,
        id: self.id,
        children: self.children,
        on_click: self.on_click.clone(),
        on_hover: self.on_hover.clone(),
        style: self.style,
    })
}

#[element]
#[derive(Default, Debug)]
pub struct Input {
    pub on_click: Handler<Input<'a>>,
    pub text: String,
    pub placeholder: &'a str,
}

impl ElementWidget for Input<'_> {
    fn render(&self, writer: &mut crate::Writer) {
        if self.text.is_empty() {
            writer.write(&format!(
                "{}{}{}",
                self.children.get_text(),
                self.placeholder,
                writer.caret()
            ));
        } else {
            writer.write(&format!(
                "{}{}{}",
                self.children.get_text(),
                self.text,
                writer.caret()
            ));
        }
    }

    fn event(&mut self, event: crossterm::event::Event, document: &mut Document) {
        match event {
            Event::Key(k) => match k.code {
                KeyCode::Backspace => {
                    if self.text.len() > 0 {
                        self.text.remove(self.text.len() - 1);
                    }
                }
                KeyCode::Enter => {
                    self.on_click.clone().call(self, event, document);
                }
                KeyCode::Char(c) => {
                    self.text.push(c);
                }
                _ => {}
            },
            _ => {}
        }
    }
}

#[component]
pub fn input<'a>(
    class: &'a str,
    id: &'a str,
    placeholder: &'a str,
    text: String,
    children: Children,
    style: Style,
    on_click: Handler<Input<'a>>,
) -> Box<Input<'a>> {
    Box::new(Input {
        class: self.class,
        id: self.id,
        children: self.children,
        style: self.style,
        on_click: self.on_click,
        placeholder: self.placeholder,
        text: self.text,
    })
}
