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
//! #[component]
//! pub fn App() -> Element {
//!     let count = State::new(0);
//!
//!     rsx! {
//!         button {
//!             class: "btn",
//!
//!             on_click: fn(_, _, _) @count {
//!                 count += 1;
//!             },
//!
//!             "The current count is: {count}"
//!         }
//!     }
//! }
//! fn main() {
//!     launch!(App);
//! }
//! ```
//!
//! ## Modules
//! - `ui` - Contains all user interface components, enabling rich CLI experiences.
//! - `utils` - Utility functions for common TUI tasks such as clearing the screen.

use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{AddAssign, SubAssign},
    sync::{Arc, Mutex},
};

use crossterm::event::Event;
use ui::{Style, StyleCore};
use utils::VisibleLength;

pub mod css;
pub mod examples;
pub mod macros;
pub mod rsx;
pub mod ui;
pub mod utils;

pub mod prelude {
    pub use crate::ui::Color::*;
    pub use crate::ui::Font::*;
    pub use crate::ui::Number::{Auto, Center};
    pub use crate::Instruction::*;
    pub use crate::{self as osui, css, ersx, launch, rsx, ui::*, Handler};
    pub use crate::{style, Component, Document};
    pub use crate::{Children, Element, ElementCore, ElementWidget};
    pub use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
    pub use osui_element::component;
    pub fn sleep(ms: u64) {
        std::thread::sleep(std::time::Duration::from_millis(ms));
    }
    pub fn use_state<'a, T>(v: T) -> crate::State<T> {
        crate::State::new(v)
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
    fn event(&mut self, event: crossterm::event::Event, document: &mut Document) {
        _ = (event, document)
    }
    fn initialize(&mut self, document: &mut Document) {
        _ = document
    }
}

pub trait Component: std::fmt::Debug {
    type Element;
    fn create_element(self) -> Self::Element;
}

//////////////////////////////////////////////////////////////////////////////////////////////////
/// Structs
//////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Handler<T>(pub Arc<Mutex<dyn FnMut(&mut T, crossterm::event::Event, &mut Document)>>);

pub struct Document {
    element: *mut Element,
    running: Option<bool>,
    pub css: ui::Css,
}

#[derive(Debug)]
pub struct Writer {
    css: *const ui::Css,
    style: ui::StyleElement,
    focused: bool,
    size: (u16, u16),
    pos: (u16, u16),
    written: (u16, u16),
    parent: Option<*mut Writer>,
}

pub struct State<T>(Arc<Mutex<T>>);

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
            running: None,
            css: HashMap::new(),
        }
    }
    pub fn exit(&mut self) {
        self.running = Some(false);
    }
    pub fn restart(&mut self) {
        self.running = Some(true);
    }
    pub fn get_root<T>(&self) -> &mut Box<T> {
        unsafe { &mut *(self.element as *mut Element as *mut Box<T>) }
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
            let (width, height) = crossterm::terminal::size().unwrap();
            let mut writer = Writer {
                css: &self.css,
                focused: true,
                style: ui::StyleElement {
                    // outline: (true, true),
                    ..Default::default()
                },
                size: (width, height),
                pos: (1, 1),
                written: (0, 0),
                parent: None,
            };
            let mut frame = writer.new_frame();
            utils::clear();
            frame.render(true, unsafe { &*self.element });
            writer.after_render(width, height);
            utils::flush();
        }
    }
    pub fn set_css(&mut self, css: ui::Css) {
        self.css = css;
    }
    pub fn clear_css(&mut self) {
        self.css.clear();
    }
    pub fn draw(&mut self, element: Element) {
        unsafe {
            *self.element = element;
            (*self.element).initialize(self);
        }
    }
    pub fn run(&mut self) -> bool {
        // Set up the screen
        utils::hide_cursor();
        crossterm::terminal::enable_raw_mode().unwrap();
        utils::clear();

        self.running = None;
        let element = unsafe { &mut *self.element };
        self.render();
        element.initialize(self);

        // Send initial event
        element.event(crossterm::event::Event::FocusGained, self);

        while self.running == None {
            self.render();
            element.event(crossterm::event::read().unwrap(), self);
        }

        utils::show_cursor();
        crossterm::terminal::disable_raw_mode().unwrap();
        utils::clear();

        self.running.unwrap()
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

    pub fn set_static(&mut self, text: String) {
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
    pub fn set_index(&mut self, index: usize) {
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
    pub fn index_mut(&mut self) -> Option<&mut usize> {
        if let Children::Children(_, c) = self {
            Some(c)
        } else {
            None
        }
    }
    pub fn insert(&mut self, index: usize, element: Element) {
        if let Children::Children(children, _) = self {
            children.insert(index, element);
        }
    }
}

impl<T> Handler<T> {
    pub fn new<F>(handler_fn: F) -> Handler<T>
    where
        F: FnMut(&mut T, crossterm::event::Event, &mut Document) + 'static,
    {
        Handler(Arc::new(Mutex::new(handler_fn)))
    }

    pub fn call(&self, s: &mut T, event: Event, document: &mut Document) {
        let mut o = self.0.lock().unwrap();
        o(s, event, document);
    }

    pub fn clone(&self) -> Handler<T> {
        Handler(Arc::clone(&self.0))
    }
}

impl Writer {
    pub fn after_render(&self, width: u16, height: u16) {
        if self.style.outline.1 {
            let ol = self.style.write_outline("│");
            let hl = "─".repeat(width as usize);
            self.write_abs_outline(&self.style.write_outline(&format!("╭{}╮", hl)), (0, 0));
            for i in 1..height+1 {
                self.write_abs_outline(&ol, (0, i));
                self.write_abs_outline(&ol, (width + 1, i));
            }
            self.write_abs_outline(&self.style.write_outline(&format!("╰{}╯", hl)), (0, height + 1));
        }
    }

    pub fn write(&mut self, s: &str) {
        self.write_abs(&self.style.write(&s), (0, 0));
    }

    pub fn write_abs_outline(&self, s: &str, pos: (u16, u16)) {
        if let Some(parent) = self.parent {
            let parent = unsafe { &mut *parent };
            parent.write_abs_outline(s, (self.pos.0 + pos.0, self.pos.1 + pos.1));
        } else {
            print!(
                "\x1b[{};{}H{}",
                self.pos.1 + pos.1 - 1,
                self.pos.0 + pos.0 - 1,
                self.style.write_outline(s)
            );
        }
    }

    pub fn write_abs(&mut self, s: &str, pos: (u16, u16)) {
        let width = self.get_size().0 as usize;
        let mut max_written = self.written.0;
        let mut lines_written = 0;

        let truncated_lines: Vec<_> = s
            .lines()
            .map(|l| {
                let truncated = if l.len() > width {
                    l.chars().take(width).collect::<String>()
                } else {
                    l.to_string()
                };
                max_written = max_written.max(truncated.visible_len() as u16);
                lines_written += 1;
                truncated
            })
            .collect();

        self.written.0 = max_written;
        self.written.1 += lines_written;

        if let Some(parent) = self.parent {
            let parent = unsafe { &mut *parent };
            let combined_pos = (self.pos.0 + pos.0, self.pos.1 + pos.1);
            let s_combined = truncated_lines.join("\n");
            parent.write_abs(&s_combined, combined_pos);
        } else {
            for (i, line) in truncated_lines.iter().enumerate() {
                print!(
                    "\x1b[{};{}H{}",
                    self.pos.1 + pos.1 + i as u16,
                    self.pos.0 + pos.0,
                    line
                );
            }
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

    pub fn new_frame(&mut self) -> crate::utils::Frame {
        crate::utils::Frame::new(self)
    }

    pub fn caret(&self) -> String {
        format!(
            "{}{}\x1b[0m",
            self.style.cursor.1.ansi(),
            self.style.caret.1
        )
    }
}

impl<T> Default for Handler<T> {
    fn default() -> Self {
        Handler(Arc::new(Mutex::new(
            |_: &mut T, _: crossterm::event::Event, _: &mut Document| {},
        )))
    }
}

impl<T> std::fmt::Debug for Handler<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Handler()")
    }
}

impl<T> State<T> {
    pub fn new<'a>(v: T) -> State<T> {
        State(Arc::new(Mutex::new(v)))
    }
    pub fn use_state<'a>(&'a self) -> std::sync::MutexGuard<'a, T> {
        self.0.lock().unwrap()
    }
    pub fn clone<'a>(&'a self) -> State<T> {
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

impl<T: AddAssign> AddAssign<T> for State<T> {
    fn add_assign(&mut self, rhs: T) {
        *self.0.lock().unwrap() += rhs;
    }
}

impl<T: SubAssign> SubAssign<T> for State<T> {
    fn sub_assign(&mut self, rhs: T) {
        *self.0.lock().unwrap() -= rhs;
    }
}

impl<T: PartialEq> PartialEq<T> for State<T> {
    fn eq(&self, other: &T) -> bool {
        self.use_state().eq(other)
    }
    fn ne(&self, other: &T) -> bool {
        self.use_state().ne(other)
    }
}

#[derive(Debug)]
pub enum Instruction<'a> {
    SetStyle(ui::Css),
    SetChild(usize),
    Ghost(usize),
    Load(Handler<ui::Div<'a>>),
}
