//! # State hooks module
//!
//! Provides React-like use_state hooks for managing states.
//! This module includes use_state and use_sync_state.

use std::{
    any::Any,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex, MutexGuard},
};

use super::{HookDependency, HookEffect};
use crate::component::context::Context;

/// State holder for reactive values
///
/// Similar to React's useState hook. Holds a value and tracks dependents
/// that need to be notified when the value changes.
#[derive(Debug)]
pub struct State<T> {
    /// The actual state value
    value: Arc<Mutex<T>>,
    /// Functions to call when state is updated
    dependents: Arc<Mutex<Vec<HookEffect>>>,
}

/// Guard for accessing and potentially modifying state
///
/// Dereferences to the state value. When dropped after modification,
/// automatically triggers all dependent effects.
pub struct Inner<'a, T> {
    value: MutexGuard<'a, T>,
    dependents: Arc<Mutex<Vec<HookEffect>>>,
    updated: bool,
}

impl<T: Clone> State<T> {
    /// Gets a cloned copy of the state value
    ///
    /// Recommended over `get()` to prevent deadlocks when cloning is acceptable.
    /// "dl" stands for "deadlock-less".
    pub fn get_dl(&self) -> T {
        self.value.lock().unwrap().clone()
    }
}

impl<T> State<T> {
    /// Acquires a lock on the state for read/write access
    ///
    /// Returns an Inner guard that implements Deref and DerefMut.
    /// When dropped after modification, triggers dependent effects.
    pub fn get(&self) -> Inner<'_, T> {
        Inner {
            value: self.value.lock().unwrap(),
            dependents: self.dependents.clone(),
            updated: false,
        }
    }

    /// Sets the state value and triggers dependents
    pub fn set(&self, v: T) {
        *self.value.lock().unwrap() = v;
        self.update();
    }

    /// Notifies all dependents of an update
    pub fn update(&self) {
        for d in self.dependents.lock().unwrap().iter() {
            d.call();
        }
    }

    /// Clones the State handle (not the value)
    pub fn clone(&self) -> Self {
        Self {
            dependents: self.dependents.clone(),
            value: self.value.clone(),
        }
    }
}

impl<T: Display> Display for State<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.value.lock().unwrap())
    }
}

/// Inner implements Deref for read access and DerefMut for write access
/// On drop after mutation, automatically triggers dependent effects
impl<T> Drop for Inner<'_, T> {
    fn drop(&mut self) {
        if self.updated {
            for d in self.dependents.lock().unwrap().iter() {
                d.call();
            }
        }
    }
}

impl<T> Deref for Inner<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Inner<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.updated = true;
        &mut self.value
    }
}

impl<T: Send + Sync> HookDependency for State<T> {
    fn on_update(&self, hook: HookEffect) {
        self.dependents.lock().unwrap().push(hook);
    }
}

/// Synchronizes state with events from the context
///
/// Creates state that is automatically updated when events are emitted
/// to the context. The decoder function converts events to state values.
pub fn use_sync_state<
    T: Send + Sync + 'static,
    E: Any + 'static,
    D: Fn(&E) -> T + Send + Sync + 'static,
>(
    cx: &Arc<Context>,
    v: T,
    decoder: D,
) -> State<T> {
    let state = use_state(v);

    cx.on_event({
        let state = state.clone();
        move |_, v: &E| state.set(decoder(v))
    });

    state
}

/// Creates a new state value
///
/// Returns a State that can be read and written from multiple threads.
pub fn use_state<T>(v: T) -> State<T> {
    State {
        value: Arc::new(Mutex::new(v)),
        dependents: Arc::new(Mutex::new(Vec::new())),
    }
}
