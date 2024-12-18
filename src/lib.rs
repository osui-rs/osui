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
    pub use crate::ui::Instruction::*;
    pub use crate::ui::Number::{Auto, Center};
    pub use crate::{self as osui, css, ersx, launch, rsx, ui::*, Handler};
    pub use crate::{call, Children, Element, ElementCore, ElementWidget};
    pub use crate::{style, Document, State};
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
    fn get_id(&self) -> String;
    fn get_class(&self) -> String;
    fn get_style(&self) -> &Style;
}

pub trait ElementWidget: ElementCore + std::fmt::Debug {
    fn render(&self, writer: &mut Writer);
    fn event(&mut self, event: crossterm::event::Event, document: &Document) {
        _ = (event, document)
    }
    fn initialize(&mut self, document: &mut Document) {
        _ = document
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////
/// Structs
//////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Handler<T>(pub Arc<Mutex<dyn FnMut(&mut T, crossterm::event::Event, &Document)>>);

pub struct Document {
    element: *mut Element,
    running: *mut Option<bool>,
    pub css: ui::Css,
}

#[derive(Debug)]
pub struct Writer {
    css: *const ui::Css,
    style: ui::StyleElement,
    focused: bool,
    size: (u16, u16),
    absolute: (u16, u16),
    written: (u16, u16),
}

//////////////////////////////////////////////////////////////////////////////////////////////////
/// Enums
//////////////////////////////////////////////////////////////////////////////////////////////////

pub enum Children {
    None,
    Text(Arc<dyn Fn() -> String>),
    StaticText(String),
    Children(Vec<Element>, usize),
}

//////////////////////////////////////////////////////////////////////////////////////////////////
/// Implementations
//////////////////////////////////////////////////////////////////////////////////////////////////

impl Document {
    pub fn with_elem(element: &mut Element) -> Document {
        Document {
            element,
            running: &mut None,
            css: HashMap::new(),
        }
    }
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
        if !self.element.is_null() {
            // unsafe {
            //     (*self.element).set_styling(&self.css);
            // }
            let (width, height) = crossterm::terminal::size().unwrap();
            let mut frame = utils::Frame::new((0, 0), (width, height), &self.css);
            utils::clear();
            frame.render(true, unsafe { &*self.element });
            utils::flush();
        }
    }
    pub fn set_css(&mut self, css: ui::Css) {
        self.css = css;
    }
    pub fn draw(&mut self, element: &mut Element) {
        self.element = element;
    }
    pub fn run(&mut self) -> bool {
        unsafe { *self.running = None }
        let element = unsafe { &mut *self.element };
        element.initialize(self);

        // Send initial event
        element.event(crossterm::event::Event::FocusGained, self);

        utils::hide_cursor();
        crossterm::terminal::enable_raw_mode().unwrap();
        utils::clear();

        while unsafe { *self.running } == None {
            self.render();
            element.event(crossterm::event::read().unwrap(), self);
        }

        utils::show_cursor();
        crossterm::terminal::disable_raw_mode().unwrap();
        utils::clear();

        unsafe { (*self.running).unwrap() }
    }
}

impl Default for Children {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Debug for Children {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Self::None => String::new(),
            Self::Children(a, b) => format!("({b}, {a:?})"),
            Self::Text(a) => format!("{:?}", a()),
            Self::StaticText(a) => format!("static({a:?})"),
        })
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
            Children::Text(text) => text(),
            Children::StaticText(text) => text.clone(),
            _ => String::new(),
        }
    }
    pub fn set_text(&mut self, text: Arc<dyn Fn() -> String>) {
        match self {
            Children::Text(t) => {
                *t = text;
            }
            _ => {}
        }
    }
    pub fn set_text_force(&mut self, text: Arc<dyn Fn() -> String>) {
        match self {
            Children::Text(t) => {
                *t = text;
            }
            _ => {
                *self = Children::Text(text);
            }
        }
    }
    pub fn set_static_force(&mut self, text: String) {
        *self = Children::StaticText(text);
    }
    pub fn add_child(&mut self, element: Element) {
        if let Children::Children(children, _) = self {
            children.push(element);
        }
    }
    pub fn get_child(&mut self) -> (Option<&mut Element>, usize) {
        if let Children::Children(children, child) = self {
            let l = children.len();
            (children.get_mut(*child), l)
        } else {
            (None, 0)
        }
    }
    pub fn get_child_idx(&mut self) -> (Option<(&mut Element, &mut usize)>, usize) {
        if let Children::Children(children, child) = self {
            let l = children.len();
            if let Some(d) = children.get_mut(*child) {
                (Some((d, child)), l)
            } else {
                (None, 0)
            }
        } else {
            (None, 0)
        }
    }
    pub fn set_child(&mut self, index: usize) {
        if let Children::Children(_, child) = self {
            *child = index;
        }
    }
    pub fn len(&self) -> usize {
        if let Children::Children(children, _) = self {
            children.len()
        } else {
            0
        }
    }
    pub fn index(&self) -> usize {
        if let Children::Children(_, c) = self {
            *c
        } else {
            0
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
        css: &ui::Css,
    ) -> Writer {
        Writer {
            css,
            style,
            focused,
            size,
            absolute,
            written: (0, 0),
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
            let line_len = line.len() as u16;
            if line_len > self.written.0 {
                self.written.0 = line_len;
            }
            self.written.1 += 1;
        }
    }

    pub fn get_focused(&self) -> bool {
        self.focused
    }

    pub fn get_size(&self) -> (u16, u16) {
        (
            self.style
                .width
                .1
                .as_size(self.size.0, self.size.0, self.style.outline.1),
            self.style
                .height
                .1
                .as_size(self.size.1, self.size.1, self.style.outline.1),
        )
    }

    pub fn get_size_root(&self) -> (u16, u16) {
        (
            self.style
                .width
                .1
                .as_size(self.written.0, self.size.0, self.style.outline.1),
            self.style
                .height
                .1
                .as_size(self.written.1, self.size.1, self.style.outline.1),
        )
    }

    pub fn new_frame(&self) -> crate::utils::Frame {
        crate::utils::Frame::new(self.absolute, self.get_size(), unsafe { &*self.css })
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

pub struct State<T>(Arc<Mutex<T>>);
impl<T> State<T> {
    pub fn new<'a>(v: T) -> State<T> {
        State(Arc::new(Mutex::new(v)))
    }
    pub fn use_state<'a>(&'a self) -> std::sync::MutexGuard<'a, T> {
        self.0.lock().unwrap()
    }
    pub fn copy_state<'a>(&'a self) -> State<T> {
        State(Arc::clone(&self.0))
    }
}

impl<T: std::fmt::Display> std::fmt::Display for State<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Acquire the lock, handle potential poisoning
        match self.0.lock() {
            Ok(guard) => write!(f, "{}", *guard),
            Err(poisoned) => write!(f, "<Poisoned: {}>", *poisoned.into_inner()),
        }
    }
}
