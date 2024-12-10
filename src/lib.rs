//! # OSUI
//!
//! A terminal user interface (TUI) library providing customizable components
//! to build command-line interfaces in Rust. OSUI enables users to create
//! interactive CLI applications with various UI elements and handle keyboard
//! input for real-time events.
//!
//! ## Example Usage
//!
//! ```rust
//! use osui::prelude::*;
//!
//! launch(rsx! {
//!     text { "Hello, World!" }
//! });
//! ```
//!
//! ## Modules
//! - `ui` - Contains all user interface components, enabling rich CLI experiences.
//! - `utils` - Utility functions for common TUI tasks such as clearing the screen.

use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{Arc, Mutex},
};

use ui::Style;

pub mod css;
pub mod examples;
pub mod macros;
pub mod rsx;
pub mod ui;
pub mod utils;

pub mod prelude {
    pub use crate::ui::Color::*;
    pub use crate::ui::Font::*;
    pub use crate::ui::Number::*;
    pub use crate::{self as osui, css, ersx, launch, rsx, ui::*, Handler};
    pub use crate::{call, Children, Element, ElementCore, ElementWidget};
    pub use crate::{style, Document};
    pub use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
    pub fn sleep(ms: u64) {
        std::thread::sleep(std::time::Duration::from_millis(ms));
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////
// Type aliases
//////////////////////////////////////////////////////////////////////////////////////////////////

pub type Element = Box<dyn ElementWidget>;

//////////////////////////////////////////////////////////////////////////////////////////////////
/// Traits
//////////////////////////////////////////////////////////////////////////////////////////////////

pub trait ElementCore {
    fn get_element_by_id(&mut self, id: &str) -> Option<&mut Element>;
    fn set_styling(&mut self, styling: &HashMap<crate::ui::StyleName, crate::ui::Style>);
    fn get_id(&self) -> String;
    fn get_style(&self) -> &Style;
}

pub trait ElementWidget: ElementCore + std::fmt::Debug {
    fn render(&self, writer: &mut Writer);
    fn event(&mut self, event: crossterm::event::Event, document: &Document) {
        _ = (event, document)
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////
/// Structs
//////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Handler<T>(pub Arc<Mutex<dyn FnMut(&mut T, crossterm::event::Event, &Document)>>);

pub struct Document {
    element: *mut Element,
    running: *mut Option<bool>,
}

#[derive(Debug)]
pub struct Writer {
    style: ui::StyleElement,
    focused: bool,
    size: (u16, u16),
    absolute: (u16, u16),
}

//////////////////////////////////////////////////////////////////////////////////////////////////
/// Enums
//////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum Children {
    None,
    Text(String),
    Children(Vec<Element>, usize),
}

//////////////////////////////////////////////////////////////////////////////////////////////////
/// Implementations
//////////////////////////////////////////////////////////////////////////////////////////////////

impl Document {
    pub fn exit(&self) {
        if !self.running.is_null() {
            unsafe {
                *self.running = Some(false);
            }
        }
    }
    pub fn restart(&self) {
        if !self.running.is_null() {
            unsafe {
                *self.running = Some(true);
            }
        }
    }
    pub fn get_element_by_id<T>(&self, id: &str) -> Option<&mut Box<T>> {
        if let Some(e) = self.get_element_by_id_raw(id) {
            Some(unsafe { &mut *(e as *mut Element as *mut Box<T>) })
        } else {
            None
        }
    }
    pub fn get_element_by_id_raw(&self, id: &str) -> Option<&mut Element> {
        if !self.element.is_null() {
            unsafe {
                if (*self.element).get_id() == id {
                    Some(&mut *self.element)
                } else {
                    (*self.element).get_element_by_id(id)
                }
            }
        } else {
            None
        }
    }
    pub fn render(&self) {
        if !self.running.is_null() {
            let (width, height) = crossterm::terminal::size().unwrap();
            let mut frame = utils::Frame::new((0, 0), (width, height));
            utils::clear();
            frame.render(true, unsafe { &*self.element });
            utils::flush();
        }
    }
    pub fn draw(&mut self, element: &mut Element) {
        self.element = element;
    }
    pub fn use_state<T>(&self, name: &str) -> &mut T {
        &mut self
            .get_element_by_id::<crate::ui::DataHolder<T>>(name)
            .unwrap()
            .data
    }
}

impl Default for Children {
    fn default() -> Self {
        Self::None
    }
}

impl Children {
    pub fn is_none(&self) -> bool {
        match self {
            Children::None => true,
            _ => false,
        }
    }
    pub fn get_text(&self) -> String {
        match self {
            Children::Text(text) => text.clone(),
            _ => String::new(),
        }
    }
    pub fn get_text_mut(&mut self) -> Option<&mut String> {
        match self {
            Children::Text(text) => Some(text),
            _ => None,
        }
    }
    pub fn set_text(&mut self, text: &str) {
        match self {
            Children::Text(t) => {
                *t = text.to_string();
            }
            _ => {}
        }
    }
    pub fn set_text_force(&mut self, text: &str) {
        match self {
            Children::Text(t) => {
                *t = text.to_string();
            }
            _ => {
                *self = Children::Text(text.to_string());
            }
        }
    }
    pub fn add_child(&mut self, element: Element) {
        if let Children::Children(children, _) = self {
            children.push(element);
        }
    }
    pub fn get_child(&mut self) -> Option<&mut Element> {
        if let Children::Children(children, child) = self {
            children.get_mut(*child)
        } else {
            None
        }
    }
}

impl<T> Handler<T> {
    pub fn new<F>(handler_fn: F) -> Handler<T>
    where
        F: FnMut(&mut T, crossterm::event::Event, &Document) + 'static,
    {
        Handler(Arc::new(Mutex::new(handler_fn)))
    }
}

impl Writer {
    pub fn new(
        focused: bool,
        style: ui::StyleElement,
        absolute: (u16, u16),
        size: (u16, u16),
    ) -> Writer {
        Writer {
            style,
            focused,
            size,
            absolute,
        }
    }

    pub fn write(&mut self, s: &str) {
        for (i, line) in s.lines().enumerate() {
            print!(
                "\x1B[{};{}H{}",
                self.absolute.1 + 1 + i as u16,
                self.absolute.0 + 1,
                self.style.write(line)
            );
        }
    }

    pub fn get_focused(&self) -> bool {
        self.focused
    }

    pub fn get_size(&self) -> (u16, u16) {
        (
            self.style.width.as_size(self.size.0, self.style.outline),
            self.style.height.as_size(self.size.1, self.style.outline),
        )
    }

    pub fn new_frame(&self) -> crate::utils::Frame {
        crate::utils::Frame::new(self.absolute, self.get_size())
    }
}

impl<T> Default for Handler<T> {
    fn default() -> Self {
        Handler(Arc::new(Mutex::new(
            |_: &mut T, _: crossterm::event::Event, _: &Document| {},
        )))
    }
}

impl<T> std::fmt::Debug for Handler<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Handler()")
    }
}

pub fn run(element: &mut Element) -> bool {
    let document = Document {
        element: element,
        running: &mut None,
    };

    // Send initial event
    element.event(crossterm::event::Event::FocusGained, &document);

    utils::hide_cursor();
    crossterm::terminal::enable_raw_mode().unwrap();
    utils::clear();

    while unsafe { *document.running } == None {
        document.render();
        element.event(crossterm::event::read().unwrap(), &document);
    }

    utils::show_cursor();
    crossterm::terminal::disable_raw_mode().unwrap();
    utils::clear();

    unsafe { (*document.running).unwrap() }
}
