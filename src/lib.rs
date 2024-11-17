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
//! use osui::{self, rsx, ui::*};
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

pub mod app;
pub mod macros;
pub mod ui;
pub mod utils;

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

pub trait ElementComponent: Send + Sync {
    fn get_data(&self) -> (Value<usize>, usize, String);
    fn update_data(&mut self, width: usize, height: usize);
    fn get_element_by_id(&mut self, id: &str) -> Option<&mut Box<dyn Element>>;
    fn get_child(&mut self) -> Option<&mut Box<dyn Element>>;
}

pub trait Element: std::fmt::Debug + Send + Sync + ElementComponent {
    fn render(&self, state: State) -> String {
        _ = state;
        String::new()
    }

    fn event(&mut self, event: crossterm::event::Event, state: &StateChanger) {
        (_, _) = (event, state)
    }
}

pub struct Handler<T>(pub Box<dyn FnMut(&T, crossterm::event::Event, &StateChanger) + Send>);

impl<T> Handler<T> {
    pub fn new<F>(handler_fn: F) -> Handler<T>
    where
        F: FnMut(&T, crossterm::event::Event, &StateChanger) + 'static + Send,
    {
        Handler(Box::new(handler_fn))
    }
}

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

#[derive(Debug)]
pub enum Children {
    None,
    Text(String),
    Children(Vec<Box<dyn Element>>, usize),
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
}
