//! # State Management and Hooks Module
//!
//! Provides React-like hooks for managing component state and side effects.
//! This module includes useState, useEffect, useMount, and state synchronization hooks.

use std::{
    fmt::{Debug, Formatter, Result as FmtResult},
    sync::{Arc, Mutex},
};

mod effect;
mod mount;
mod state;
pub use effect::*;
pub use mount::*;
pub use state::*;

/// Effect callback that can be triggered by state changes
#[derive(Clone)]
pub struct HookEffect(Arc<Mutex<dyn FnMut() + Send + Sync>>);

/// Trait for values that can be tracked as dependencies in hooks
pub trait HookDependency: Send + Sync {
    /// Register an effect to be triggered on updates
    fn on_update(&self, hook: HookEffect);
}

impl HookEffect {
    /// Creates a new effect from a function
    pub fn new<F: Fn() + Send + Sync + 'static>(f: F) -> Self {
        Self(Arc::new(Mutex::new(f)))
    }

    /// Executes the effect function
    pub fn call(&self) {
        (self.0.lock().unwrap())()
    }
}

impl Debug for HookEffect {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "HookEffect")
    }
}
