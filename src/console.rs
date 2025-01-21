//! Console module
//!
//! This module provides utilities for managing terminal interactions, including event handling,
//! rendering, and terminal state management.

use crate::{Element, Frame};

/// Represents the console state, containing a frame for rendering and a mouse capture flag.
pub struct Console(Frame, bool);

/// Enum representing various events that can occur in the console.
#[derive(Debug, Clone)]
pub enum Event {
    /// A keyboard event.
    Key(crossterm::event::KeyEvent),
    /// A terminal resize event with new dimensions (width, height).
    Resize(u16, u16),
    /// A mouse event.
    Mouse(crossterm::event::MouseEvent),
    /// A paste event with the pasted content.
    Paste(String),
    /// An event indicating the terminal gained focus.
    FocusGained,
    /// An event indicating the terminal lost focus.
    FocusLost,
}

/// Initializes the console with raw mode enabled and optionally mouse capture.
///
/// # Arguments
/// * `mouse` - A boolean flag indicating whether to enable mouse capture.
///
/// # Returns
/// A `Console` instance wrapped in a `Result`.
pub fn init(mouse: bool) -> crate::Result<Console> {
    crossterm::terminal::enable_raw_mode()?;
    crate::utils::clear()?;
    crate::utils::hide_cursor()?;
    if mouse {
        crossterm::execute!(std::io::stdout(), crossterm::event::EnableMouseCapture)?;
    }
    Ok(Console(Frame::new(crossterm::terminal::size()?), mouse))
}

impl Console {
    /// Renders a user interface element with an optional event.
    ///
    /// # Arguments
    /// * `ui` - The UI element to render.
    /// * `event` - An optional event to pass to the UI element.
    ///
    /// # Returns
    /// A `Result` indicating success or failure.
    pub fn draw(&mut self, ui: Element, event: Option<Event>) -> crate::Result<()> {
        self.0.clear()?;
        ui(&mut self.0, event)
    }

    /// Runs the console loop, rendering the UI and handling events.
    ///
    ///
    /// # Arguments
    /// * `ui` - The UI element to render.
    ///
    /// # Returns
    /// A `Result` indicating success or failure.
    pub fn run(&mut self, ui: Element) -> crate::Result<()> {
        self.draw(ui.clone(), None)?;
        loop {
            let event = read()?;
            if let Event::Resize(w, h) = event {
                self.0.width = w;
                self.0.height = h;
            } else if let Event::Mouse(crossterm::event::MouseEvent {
                kind: crossterm::event::MouseEventKind::Moved,
                row,
                column,
                ..
            }) = event
            {
                self.0.mouse_pos = Some((column, row));

                self.draw(ui.clone(), None)?;
            } else {
                self.draw(ui.clone(), Some(event))?;
            }
        }
    }

    /// Ends the console session, restoring the terminal state.
    ///
    /// # Returns
    /// A `Result` indicating success or failure.
    pub fn end(&self) -> crate::Result<()> {
        if self.1 {
            crossterm::execute!(std::io::stdout(), crossterm::event::DisableMouseCapture)?;
        }
        crossterm::terminal::disable_raw_mode()?;
        crate::utils::clear()?;
        crate::utils::show_cursor()
    }

    /// Retrieves the current terminal size.
    ///
    /// # Returns
    /// A tuple containing the width and height of the terminal.
    pub fn size(&self) -> (u16, u16) {
        (self.0.width, self.0.height)
    }
}

/// Reads an event from the terminal.
///
/// # Returns
/// An `Event` wrapped in a `Result`.
pub fn read() -> crate::Result<Event> {
    let event = crossterm::event::read()?;

    Ok(match event {
        crossterm::event::Event::Key(k) => Event::Key(k),
        crossterm::event::Event::Resize(w, h) => Event::Resize(w, h),
        crossterm::event::Event::FocusGained => Event::FocusGained,
        crossterm::event::Event::FocusLost => Event::FocusLost,
        crossterm::event::Event::Mouse(m) => Event::Mouse(m),
        crossterm::event::Event::Paste(p) => Event::Paste(p),
    })
}

/// Attempts to read an event from the terminal without blocking.
///
/// # Returns
/// An `Option` containing an `Event` if one is available.
pub fn try_read() -> Option<Event> {
    if crossterm::event::poll(std::time::Duration::ZERO).unwrap_or(false) {
        read().ok()
    } else {
        None
    }
}
