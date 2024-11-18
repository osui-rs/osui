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
//! osui::app::run(&mut rsx! {
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

/// The `prelude` module provides commonly used imports and macros to simplify the development
/// of UI elements. It re-exports commonly used traits, structs, and utility functions to make
/// the UI framework easier to use.
pub mod prelude {
    pub use crate::ui::Color::*;
    pub use crate::{app::run, css, rsx, rsx_elem, ui::*, Handler};
    pub use crate::{Children, Element, ElementCore, State, Value};
    pub use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
    pub fn sleep(ms: u64) {
        std::thread::sleep(std::time::Duration::from_millis(ms));
    }
}

pub mod prelude_extra{
    pub use crate::ui::Color::*;
    pub use crate::{app::run, css, rsx, rsx_elem, ui::*, Handler, StateChanger};
    pub use crate::{Children, Component, Element, ElementCore, State, Value, run_handler};
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
    fn render(&self, state: State) -> String {
        _ = state;
        String::new()
    }

    fn event(&mut self, event: Event, state: &StateChanger) {
        (_, _) = (event, state)
    }
}

pub type Element = Box<dyn Component>;

//////////////////////////////////////////////////////////////////////////////////////////////////
/// Handler Struct
//////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Handler<T>(pub Arc<Mutex<dyn FnMut(&mut T, Event, &StateChanger) + Send + Sync>>);

impl<T> Handler<T> {
    pub fn new<F>(handler_fn: F) -> Handler<T>
    where
        F: FnMut(&mut T, Event, &StateChanger) + 'static + Send + Sync,
    {
        Handler(Arc::new(Mutex::new(handler_fn)))
    }
}

impl<T> Default for Handler<T> {
    fn default() -> Self {
        Handler(Arc::new(Mutex::new(
            |_: &mut T, _: Event, _: &StateChanger| {},
        )))
    }
}

impl<T> std::fmt::Debug for Handler<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Handler()")
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////
/// State Enum and StateChanger Struct
//////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    Exit,
    Rebuild,
    Normal,
    Hover,
    Custom(usize),
}

impl State {
    pub fn from_u32(s: u32) -> State {
        match s {
            0 => State::Exit,
            1 => State::Rebuild,
            2 => State::Normal,
            3 => State::Hover,
            _ => State::Custom((s - 4) as usize),
        }
    }

    pub fn to_u32(&self) -> u32 {
        match self {
            State::Exit => 0,
            State::Rebuild => 1,
            State::Normal => 2,
            State::Hover => 3,
            State::Custom(s) => (4 + s) as u32,
        }
    }
}

pub struct StateChanger(*mut u32);

impl StateChanger {
    pub fn set_state(&self, s: State) {
        unsafe {
            (*self.0) = s.to_u32();
        }
    }

    pub fn get_state(&self) -> State {
        State::from_u32(unsafe { *self.0 })
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
