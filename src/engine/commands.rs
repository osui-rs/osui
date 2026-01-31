//! # Commands Module
//!
//! Defines built-in commands for controlling the engine.

use crate::engine::Command;

/// Command to stop the engine and terminate the application
pub struct Stop;

impl Command for Stop {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
