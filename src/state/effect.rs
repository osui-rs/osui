//! # Effect hooks module
//!
//! Provides React-like use_effect hooks for managing effects.
//! This module includes use_effect and use_sync_effect.

use std::sync::{Arc, Mutex};

use super::{state::State, HookDependency, HookEffect};
use crate::component::context::Context;

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
