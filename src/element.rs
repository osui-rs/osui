//! # Element Module
//!
//! This module defines the `Element` trait and associated types for creating,
//! configuring, and updating UI elements in OSUI. Elements are the building
//! blocks of the TUI, each with properties such as size and position and
//! methods for rendering and updating.

use dyn_clone::{clone_trait_object, DynClone};

/// Enum representing the size of an `Element`.
///
/// - `Default(usize)` - Uses a default size for the element.
/// - `Custom(usize)` - Allows specifying a custom size for the element.
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

    /// Retrieves the size as an `usize`.
    ///
    /// # Returns
    ///
    /// The size value, either the default or custom size.
    pub fn get_size(&self) -> T {
        match self {
            Value::Custom(s) => *s,
            Value::Default(s) => *s,
        }
    }

    /// Attempts to set the size of an `Element` if it is currently set to `Default`.
    ///
    /// If the size is `Custom`, this function will not modify it.
    ///
    /// # Arguments
    ///
    /// * `size` - The size to set for the `Element`.
    pub fn try_set_size(&mut self, value: T) {
        if let Value::Default(_) = *self {
            *self = Value::Default(value);
        }
    }
}

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

    fn get_id(&mut self) -> String;

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
    /// * `_state` - The current state of the element; if one or higher, it indicates the element is active.
    /// * `_k` - The key input triggering the update.
    ///
    /// # Returns
    ///
    /// An `UpdateResponse` enum indicating the result of the update.
    fn event(&mut self, _state: usize, _k: crate::key::Key) -> UpdateResponse {
        UpdateResponse::None
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
    pub width: Value<usize>,
    /// Height of the `Element`, which can be default or custom.
    pub height: Value<usize>,
    /// Children of the `Element`
    pub children: Vec<Box<dyn Element>>,
    /// Active Child of the `Element`
    pub child: usize,
    /// Text of the `Element`
    pub text: String,
    /// Identifier of the `Element`
    pub id: String,
}

/// Enum representing the possible responses from an element update.
#[derive(Debug, Clone, PartialEq)]
pub enum UpdateResponse {
    /// Indicates that the update is complete.
    Done,
    /// Indicates no response.
    None,
    /// Issues a single command.
    Command(Command),
    /// Issues a list of commands.
    CommandList(Vec<Command>),
}

/// Enum defining commands that can be issued by an `Element`.
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    /// Renders the element with the specified state.
    Render(usize),
    /// Exits the application.
    Exit,
    /// Updates the element with the specified state.
    Update(usize),
    /// Pauses execution for the specified duration in milliseconds.
    Sleep(u64),
}
