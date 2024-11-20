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
//! run(&mut rsx! {
//!     text { "Hello, World!" }
//! });
//! ```
//!
//! ## Modules
//! - `app` - The main function for rendering.
//! - `ui` - Contains all user interface components, enabling rich CLI experiences.
//! - `utils` - Utility functions for common TUI tasks such as clearing the screen.

use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{Arc, Mutex},
};

use crossterm::event::Event;

pub mod app;
pub mod macros;
pub mod ui;
pub mod utils;

pub use app::run;

/// The `prelude` module provides commonly used imports and macros to simplify the development
/// of UI elements. It re-exports commonly used traits, structs, and utility functions to make
/// the UI framework easier to use.
pub mod prelude {
    pub use crate::ui::Color::*;
    pub use crate::{self as osui, css, rsx, rsx_elem, ui::*, Handler};
    pub use crate::{Element, Value};
    // useful for Element making
    pub use crate::{run_handler, Children, Component, ElementCore};
    pub use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
    pub fn sleep(ms: u64) {
        std::thread::sleep(std::time::Duration::from_millis(ms));
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////
/// Value Enum
//////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy)]
pub enum Value<T: Copy + PartialEq> {
    Default(T),
    Custom(T),
}

impl<T: Copy + PartialEq> Value<T> {
    pub fn get_value(&self) -> T {
        match self {
            Value::Custom(s) => *s,
            Value::Default(s) => *s,
        }
    }

    pub fn try_set_value(&mut self, value: T) {
        if let Value::Default(_) = *self {
            *self = Value::Default(value);
        }
    }
}

impl<T: Copy + PartialEq + Default> Default for Value<T> {
    fn default() -> Self {
        Self::Default(T::default())
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////
/// ElementCore Traits
//////////////////////////////////////////////////////////////////////////////////////////////////

pub trait ElementCore: Send + Sync {
    fn get_data(&self) -> (Value<usize>, usize, String);
    fn update_data(&mut self, width: usize, height: usize);
    fn get_element_by_id(&mut self, id: &str) -> Option<&mut Element>;
    fn get_child(&mut self) -> Option<&mut Element>;
    fn set_styling(&mut self, styling: &HashMap<crate::ui::StyleName, crate::ui::Style>);
}

pub trait Component: ElementCore + std::fmt::Debug {
    fn render(&self, focused: bool) -> String {
        _ = focused;
        String::new()
    }

    fn event(&mut self, event: Event) {
        _ = event
    }
}

pub type Element = Box<dyn Component>;

//////////////////////////////////////////////////////////////////////////////////////////////////
/// Handler Struct
//////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Handler<T>(pub Arc<Mutex<dyn FnMut(&mut T, Event) + Send + Sync>>);

impl<T> Handler<T> {
    pub fn new<F>(handler_fn: F) -> Handler<T>
    where
        F: FnMut(&mut T, Event) + 'static + Send + Sync,
    {
        Handler(Arc::new(Mutex::new(handler_fn)))
    }
}

impl<T> Default for Handler<T> {
    fn default() -> Self {
        Handler(Arc::new(Mutex::new(|_: &mut T, _: Event| {})))
    }
}

impl<T> std::fmt::Debug for Handler<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Handler()")
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////
/// Children Enum
//////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum Children {
    None,
    Text(String),
    Children(Vec<Element>, usize),
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
}
