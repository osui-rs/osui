//! # Mount hooks module
//!
//! Provides React-like Mount hooks for managing effects.
//! This module includes Mount, use_mount and use_mount_manual.

use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use super::{HookDependency, HookEffect};

/// Mount lifecycle hook
///
/// Tracks whether a component has been mounted and executes
/// any pending mount effects.
#[derive(Debug, Clone)]
pub struct Mount(Arc<Mutex<bool>>, Arc<Mutex<Vec<HookEffect>>>);

/// Creates a mount lifecycle hook
///
/// Returns a Mount that tracks component lifecycle and executes
/// effects after mounting.
pub fn use_mount() -> Mount {
    Mount(Arc::new(Mutex::new(true)), Arc::new(Mutex::new(Vec::new())))
}

/// Creates a manual mount lifecycle hook
///
/// Similar to use_mount but the component starts as unmounted.
/// Must call .mount() to trigger mounted effects.
pub fn use_mount_manual() -> Mount {
    Mount(
        Arc::new(Mutex::new(false)),
        Arc::new(Mutex::new(Vec::new())),
    )
}

impl HookDependency for Mount {
    fn on_update(&self, hook: HookEffect) {
        if *self.0.lock().unwrap() {
            hook.call();
        } else {
            self.1.lock().unwrap().push(hook);
        }
    }
}

impl Mount {
    /// Mark the component as mounted and execute pending effects
    pub fn mount(&self) {
        *self.0.lock().unwrap() = true;
        for hook_effect in self.1.lock().unwrap().iter() {
            hook_effect.call();
        }
        self.1.lock().unwrap().clear();
    }
}
