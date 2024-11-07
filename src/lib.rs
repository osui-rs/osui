//! # OSUI
//!
//! A terminal user interface (TUI) library providing customizable components
//! to build command-line interfaces in Rust. OSUI enables users to create
//! interactive CLI applications with various UI elements and handle keyboard
//! input for real-time updates.
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

pub mod element;
pub mod key;
pub mod macros;
mod test;
pub mod ui;
pub mod utils;

pub use element::*;
pub use utils::*;

pub mod app {
    //! Application entry point and main event loop for OSUI.
    //!
    //! Provides functions to render and update UI elements based on keyboard
    //! input. Manages cursor visibility, terminal size, and controls UI behavior
    //! using custom commands such as rendering, updating, or exiting.

    use crate::{
        clear, create_frame, flush, get_term_size, hide_cursor,
        key::{read_key, Key},
        render_to_frame, show_cursor, Command, Element, UpdateResponse, Value,
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
    /// Processes the result of `Element::update` to handle commands like rendering,
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
    fn update(elem: &mut Box<dyn Element>, state: usize, k: Key) -> bool {
        match elem.event(state, k.clone()) {
            UpdateResponse::Command(command) => run_command(command, elem, k.clone()),
            UpdateResponse::CommandList(commands) => {
                for command in commands {
                    if run_command(command, elem, k.clone()) {
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
    fn run_command(command: Command, elem: &mut Box<dyn Element>, k: Key) -> bool {
        match command {
            Command::Render(state) => {
                render(elem, state);
                false
            }
            Command::Update(state) => update(elem, state, k),
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
            let k = read_key();
            if update(elem, 1, k) {
                break;
            }
        }
    }
}

pub fn run_test() {
    test::main_();
}
