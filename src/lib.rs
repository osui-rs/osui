//! # OSUI - A TUI Library for Advanced UIs
//!
//! OSUI is a Rust library for building sophisticated Terminal User Interfaces (TUIs).
//! It provides a component-based architecture with state management, event handling,
//! and rendering capabilities for creating interactive console applications.
//!
//! ## Key Features
//!
//! - **Component System**: Build UIs using composable components
//! - **State Management**: React-like hooks for managing component state
//! - **Event Handling**: Type-safe event system with reactive updates
//! - **RSX Syntax**: Macro-based DSL for defining component hierarchies
//! - **Console Engine**: Terminal rendering with crossterm support
//!
//! ## Architecture
//!
//! - [`component`] - Component system and context management
//! - [`state`] - State management with hooks (useState, useEffect, etc.)
//! - [`engine`] - Rendering engine and command execution
//! - [`frontend`] - RSX (React-like Syntax) for component definitions
//! - [`render`] - Low-level rendering primitives
//!
//! ## Example
//!
//! ```rust,no_run
//! use osui::prelude::*;
//! use std::sync::Arc;
//!
//! #[component]
//! pub fn Counter(cx: &Arc<Context>) -> View {
//!     let count = use_state(0);
//!     
//!     Arc::new(move |ctx| {
//!         ctx.draw_text(Point { x: 0, y: 0 }, &format!("Count: {}", count.get_dl()));
//!     })
//! }
//! ```

use std::sync::Arc;

use crate::render::DrawContext;

pub mod component;
pub mod engine;
pub mod frontend;
pub mod render;
pub mod hooks;

pub mod prelude {
    //! Prelude module - Re-exports commonly used items for convenience
    pub use crate::component::{context::*, scope::*, *};
    pub use crate::engine::*;
    pub use crate::frontend::*;
    pub use crate::render::*;
    pub use crate::hooks::*;
    pub use crate::{sleep, Error, Result, View, ViewWrapper};
    pub use crossterm;
    pub use osui_macros::{component, rsx};
    pub use std::sync::{Arc, Mutex};
}

/// A View is an async function that renders content to a DrawContext.
/// It takes a mutable DrawContext and produces drawing instructions.
pub type View = Arc<dyn Fn(&mut DrawContext) + Send + Sync>;

/// A ViewWrapper is a higher-order function that wraps views.
/// It can modify or enhance how a view is rendered.
pub type ViewWrapper = Arc<dyn Fn(&mut DrawContext, View) + Send + Sync>;

/// Result type for OSUI operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for OSUI operations
#[derive(Debug, Clone)]
pub enum Error {
    /// Error that occurs when a mutex is poisoned
    PoisonError,
}

impl From<std::sync::PoisonError<std::sync::MutexGuard<'_, bool>>> for Error {
    fn from(_value: std::sync::PoisonError<std::sync::MutexGuard<'_, bool>>) -> Self {
        Error::PoisonError
    }
}

/// Sleep for the specified duration in milliseconds.
/// Useful for controlling render frame rate or delays.
pub fn sleep(delay_ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(delay_ms));
}
