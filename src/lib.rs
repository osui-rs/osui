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
//! - `element` - Defines base elements for constructing UI components.
//! - `key` - Handles keyboard input, providing key event management.
//! - `utils` - Utility functions for common TUI tasks such as clearing the screen.
//! - `ui` - Contains all user interface components, enabling rich CLI experiences.

pub mod event;
pub mod key;
pub mod macros;
mod test;
pub mod ui;
pub mod utils;
use event::{Event, EventResponse};
pub use utils::*;
pub mod app {
    //! Application entry point and main event loop for OSUI.
    //!
    //! Provides functions to render and update UI elements based on keyboard
    //! input. Manages cursor visibility, terminal size, and controls UI behavior
    //! using custom commands such as rendering, updating, or exiting.

    use crate::{
        clear, create_frame,
        event::{Command, Event, EventResponse},
        flush, get_term_size, hide_cursor,
        key::read_key,
        render_to_frame, show_cursor, Element, Value,
    };

    /// Renders a single frame of the UI to the terminal.
    ///
    /// Sets up a new frame based on the terminal's current size, updates
    /// the element dimensions, and renders the UI element to the frame.
    ///
    /// # Arguments
    ///
    /// * `elem` - A mutable reference to a boxed UI element that implements the `Element` trait.
    /// * `state` - Current state of the element, typically used to track the UI's state in the app loop.
    fn render(elem: &mut Box<dyn Element>, state: usize) {
        let (width, height) = get_term_size();
        elem.update_data(width, height);
        let mut frame: Vec<String> = create_frame(Value::Custom(width), Value::Custom(height));
        render_to_frame(state, &mut frame, elem);
        clear();
        print!("{}", frame.join(""));
        flush();
    }

    /// Updates the UI element based on keyboard input and issues any commands in response.
    ///
    /// Processes the result of `Element::event` to handle commands like rendering,
    /// updating, and exiting. Commands can be a single action or a list of actions.
    ///
    /// # Arguments
    ///
    /// * `elem` - A mutable reference to a boxed UI element.
    /// * `state` - The current UI state, used for conditional updates.
    /// * `k` - A `Key` input, typically read from the user’s keyboard input.
    ///
    /// # Returns
    ///
    /// `true` if an `Exit` command is issued, signaling the application to terminate.
    fn update(elem: &mut Box<dyn Element>, event: Event) -> bool {
        match elem.event(event) {
            EventResponse::CommandList(commands) => {
                for command in commands {
                    if run_command(command, elem) {
                        return true;
                    }
                }
                false
            }
            _ => false,
        }
    }

    /// Executes a command, performing actions such as rendering, updating,
    /// sleeping, or exiting the application.
    ///
    /// # Arguments
    ///
    /// * `command` - The command to execute, such as rendering or updating the UI.
    /// * `elem` - A mutable reference to the UI element.
    /// * `k` - A `Key` input passed along for further processing.
    ///
    /// # Returns
    ///
    /// `true` if the command is `Exit`, ending the application loop.
    fn run_command(command: Command, elem: &mut Box<dyn Element>) -> bool {
        match command {
            Command::Render(state) => {
                render(elem, state);
                false
            }
            Command::Event(event) => update(elem, event),
            Command::Sleep(duration) => {
                std::thread::sleep(std::time::Duration::from_millis(duration));
                false
            }
            Command::Exit => {
                show_cursor();
                crossterm::terminal::disable_raw_mode().unwrap();
                clear();
                true
            }
        }
    }

    /// Runs the main event loop for the application.
    ///
    /// Enables raw mode, hides the cursor, and continuously renders and updates
    /// the UI based on user input. The loop will break if the `Exit` command is triggered.
    ///
    /// # Arguments
    ///
    /// * `elem` - A mutable reference to the main UI element to be rendered and updated.
    pub fn run(elem: &mut Box<dyn Element>) {
        // Initialize terminal settings
        hide_cursor();
        crossterm::terminal::enable_raw_mode().unwrap();
        clear();
        loop {
            render(elem, 1);
            if update(elem, Event::Key(read_key())) {
                break;
            }
        }
    }
}

/// Enum representing the wether it's the default or custom.
///
/// - `Default(usize)` - Uses a default value.
/// - `Custom(usize)` - Allows specifying a custom value.
#[derive(Debug, Clone, Copy)]
pub enum Value<T: Copy> {
    Default(T),
    Custom(T),
}

impl<T: Copy> Value<T> {
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

pub trait ToValue<T: Copy> {
    fn to_value(&self) -> Value<T>;
}

use dyn_clone::{clone_trait_object, DynClone};

/// A trait for defining UI elements in OSUI.
///
/// Elements must implement methods for updating and rendering data.
pub trait Element: DynClone {
    /// Retrieves the data for the `Element`.
    ///
    /// # Returns
    ///
    /// `ElementData` containing position and size information.
    fn get_data(&self) -> ElementData;
    fn set_data(&mut self, data: ElementData);

    /// Updates the data of the `Element` based on the given dimensions.
    ///
    /// # Arguments
    ///
    /// * `_width` - The width of the terminal window or parent element.
    /// * `_height` - The height of the terminal window or parent element.
    fn update_data(&mut self, _width: usize, _height: usize);

    fn get_id(&self) -> String;

    fn get_element_by_id(&mut self, id: &str) -> Option<&mut Box<dyn Element>>;

    /// Renders the `Element` as a `String`.
    ///
    /// # Arguments
    ///
    /// * `_state` - The current state of the element; if one or higher, it indicates the element is active.
    ///              If zero, the element is just being rendered.
    ///
    /// # Returns
    ///
    /// A `String` representing the rendered output of the `Element`.
    fn render(&self, _state: usize) -> String {
        String::new()
    }

    /// Updates the `Element` based on a `Key` event and the current state.
    ///
    /// # Arguments
    ///
    /// * `_event` - The current event.
    ///
    /// # Returns
    ///
    /// An `EventResponse` enum indicating the result of the event.
    fn event(&mut self, _event: Event) -> EventResponse {
        EventResponse::None
    }
}

clone_trait_object!(Element);

/// Struct holding data relevant to an `Element`, including position and size.
pub struct ElementData {
    /// X coordinate of the `Element`.
    pub x: usize,
    /// Y coordinate of the `Element`.
    pub y: usize,
    /// Width of the `Element`, which can be default or custom.
    pub width: crate::Value<usize>,
    /// Height of the `Element`, which can be default or custom.
    pub height: crate::Value<usize>,
    /// Children of the `Element`
    pub children: Vec<Box<dyn Element>>,
    /// Active Child of the `Element`
    pub child: usize,
    /// Text of the `Element`
    pub text: String,
    /// Identifier of the `Element`
    pub id: String,
}

pub fn run_test() {
    app::run(&mut test::app());
}
