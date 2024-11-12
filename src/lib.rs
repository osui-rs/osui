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
//! - `utils` - Utility functions for common TUI tasks such as clearing the screen.
//! - `ui` - Contains all user interface components, enabling rich CLI experiences.

pub mod macros;
pub mod ui;
pub mod utils;
use std::sync::{mpsc, Arc, Mutex};

pub use utils::*;
pub mod app;

mod test;

/// Enum representing the wether it's the default or custom.
///
/// - `Default(usize)` - Uses a default value.
/// - `Custom(usize)` - Allows specifying a custom value.
#[derive(Debug, Clone, Copy)]
pub enum Value<T: Copy + PartialEq> {
    Default(T),
    Custom(T),
}

impl<T: Copy + PartialEq> Value<T> {
    /// Creates a new Value.
    ///
    /// # Returns
    ///
    /// Value::Custom(usize)
    pub fn new(value: T) -> Value<T> {
        Value::Custom(value)
    }

    /// Retrieves the value.
    ///
    /// # Returns
    ///
    /// The value by the type T.
    pub fn get_value(&self) -> T {
        match self {
            Value::Custom(s) => *s,
            Value::Default(s) => *s,
        }
    }

    /// Attempts to set the value if it is currently set to `Default`.
    ///
    /// If the size is `Custom`, this function will not modify it.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to change to.
    pub fn try_set_value(&mut self, value: T) {
        if let Value::Default(_) = *self {
            *self = Value::Default(value);
        }
    }
}

/// A trait for defining UI elements in OSUI.
///
/// Elements must implement methods for updating and rendering data.
pub trait Element: Send + Sync {
    fn get_data(&self) -> (Value<usize>, usize, String);

    fn update_data(&self, width: usize, height: usize);

    fn get_element_by_id(&self, id: &str) -> Option<std::sync::Arc<dyn Element>>;

    fn render(&self, state: State) -> String {
        _ = state;
        String::new()
    }

    fn event(&self, ch: &CommandHandler, event: crossterm::event::Event) {
        _ = (ch, event);
    }
}

pub struct Handler<T>(pub Box<dyn FnMut(&T, &CommandHandler, crossterm::event::Event) + Send>);

impl<T> Handler<T> {
    pub fn new<F>(handler_fn: F) -> Handler<T>
    where
        F: FnMut(&T, &CommandHandler, crossterm::event::Event) + 'static + Send,
    {
        Handler(Box::new(handler_fn))
    }
}

pub struct CommandHandler(mpsc::Sender<Command>);

impl CommandHandler {
    pub fn set_state(&self, state: State) {
        self.0.send(Command::SetState(state)).unwrap();
    }

    pub fn exit(&self) {
        self.0.send(Command::Exit).unwrap();
    }

    pub fn get_element_by_id(&self) {
        self.0.send(Command::Exit).unwrap();
    }

    pub fn rebuild(&self) -> bool {
        self.0.send(Command::Rebuild).is_ok()
    }
}

#[derive(Debug, Clone)]
pub enum Command {
    SetState(State),
    Exit,
    Rebuild,
}

#[derive(Debug, Clone)]
pub enum State {
    Normal,
    Hover,
    Custom(usize),
    CustomString(String),
}

pub fn run_test() {
    let i = Arc::new(Mutex::new(0));
    while app::run(test::app(Arc::clone(&i))) {}
}
