//! # Engine Module
//!
//! Provides the rendering engine and command execution system.
//! The engine is responsible for initializing components, rendering frames,
//! and handling user commands.

pub mod benchmark;
pub mod commands;
pub mod console;

pub use benchmark::*;
pub use console::*;

use std::{any::Any, sync::Arc};

use crate::component::{context::Context, ComponentImpl};
use crate::{render::Area, DrawContext, View};

/// Main engine trait for rendering and running components
pub trait Engine<Output = ()> {
    /// Runs a component to completion
    fn run<C: ComponentImpl + 'static>(&self, component: C) -> crate::Result<Output>;
    
    /// Initializes a component and returns its context
    fn init<C: ComponentImpl + 'static>(&self, component: C) -> Arc<Context>;
    
    /// Renders the current state of a component
    fn render(&self, cx: &Arc<Context>);
    
    /// Sleeps between render frames (default 16ms for ~60fps)
    fn render_delay(&self) {
        crate::sleep(16);
    }

    /// Renders a view within an area and returns the draw context
    fn render_view(&self, area: &Area, view: &View) -> DrawContext;
    
    /// Executes the drawing instructions in a draw context
    fn draw_context(&self, ctx: &DrawContext);
    
    /// Returns the command executor for this engine
    fn executor(&self) -> Arc<dyn CommandExecutor>;
}

/// Trait for commands that can be executed by the engine
pub trait Command {
    /// Returns the command as Any for downcasting
    fn as_any(&self) -> &dyn Any;
}

/// Executes commands during the application lifecycle
pub trait CommandExecutor: Send + Sync {
    /// Executes the given command
    fn execute_command(&self, command: &Arc<dyn Command>) -> crate::Result<()>;
}
