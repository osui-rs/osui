//! # State Management Module
//!
//! Provides React-like hooks for managing component state and side effects.
//! This module includes useState, useEffect, useMount, and state synchronization hooks.

use std::{
    any::Any,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex, MutexGuard},
};

use crate::component::context::Context;

/// Effect callback that can be triggered by state changes
#[derive(Clone)]
pub struct HookEffect(Arc<Mutex<dyn FnMut() + Send + Sync>>);

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

/// Mount lifecycle hook
///
/// Tracks whether a component has been mounted and executes
/// any pending mount effects.
#[derive(Debug, Clone)]
pub struct Mount(Arc<Mutex<bool>>, Arc<Mutex<Vec<HookEffect>>>);

/// Creates a new state value
///
/// Returns a State that can be read and written from multiple threads.
pub fn use_state<T>(v: T) -> State<T> {
    State {
        value: Arc::new(Mutex::new(v)),
        dependents: Arc::new(Mutex::new(Vec::new())),
    }
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

impl Debug for HookEffect {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "HookEffect")
    }
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

/// Trait for values that can be tracked as dependencies in hooks
pub trait HookDependency: Send + Sync {
    /// Register an effect to be triggered on updates
    fn on_update(&self, hook: HookEffect);
}

impl<T: Send + Sync> HookDependency for State<T> {
    fn on_update(&self, hook: HookEffect) {
        self.dependents.lock().unwrap().push(hook);
    }
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

/// Executes a function when dependencies change
///
/// Similar to React's useEffect. The provided function is executed
/// when any of the dependencies change.
pub fn use_effect<F: FnMut() + Send + Sync + 'static>(f: F, dependencies: &[&dyn HookDependency]) {
    let f = Arc::new(Mutex::new(f));
    let hook = HookEffect(Arc::new(Mutex::new({
        let f = f.clone();
        move || {
            let f = f.clone();
            std::thread::spawn(move || (f.lock().unwrap())());
        }
    })));

    for d in dependencies {
        d.on_update(hook.clone());
    }
}

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

/// Synchronizes state changes back to the context as events
///
/// Creates an effect that emits an event whenever the state changes.
/// The encoder function converts state values to events.
pub fn use_sync_effect<
    T: Send + Sync + 'static,
    Ev: Send + Sync + 'static,
    E: Fn(&State<T>) -> Ev + Send + Sync + 'static,
>(
    cx: &Arc<Context>,
    state: &State<T>,
    encoder: E,
    deps: &[&dyn HookDependency],
) {
    use_effect(
        {
            let state = state.clone();
            let cx = cx.clone();
            move || {
                let ev = encoder(&state);
                cx.emit_event(ev);
            }
        },
        deps,
    );
}
