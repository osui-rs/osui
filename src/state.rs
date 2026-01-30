use std::{
    any::Any,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex, MutexGuard},
};

use crate::component::context::Context;

#[derive(Clone)]
pub struct HookEffect(Arc<Mutex<dyn FnMut() + Send + Sync>>);

#[derive(Debug)]
pub struct State<T> {
    value: Arc<Mutex<T>>,
    dependents: Arc<Mutex<Vec<HookEffect>>>,
}

pub struct Inner<'a, T> {
    value: MutexGuard<'a, T>,
    dependents: Arc<Mutex<Vec<HookEffect>>>,
    updated: bool,
}

#[derive(Debug, Clone)]
pub struct Mount(Arc<Mutex<bool>>, Arc<Mutex<Vec<HookEffect>>>);

pub fn use_state<T>(v: T) -> State<T> {
    State {
        value: Arc::new(Mutex::new(v)),
        dependents: Arc::new(Mutex::new(Vec::new())),
    }
}

impl<T: Clone> State<T> {
    /// Gets the cloned value, recommended for preventing deadlocks
    pub fn get_dl(&self) -> T {
        self.value.lock().unwrap().clone()
    }
}

impl<T> State<T> {
    /// Gets a lock on the state for read/write access.
    pub fn get(&self) -> Inner<'_, T> {
        Inner {
            value: self.value.lock().unwrap(),
            dependents: self.dependents.clone(),
            updated: false,
        }
    }

    /// Sets the value and marks it as changed.
    pub fn set(&self, v: T) {
        *self.value.lock().unwrap() = v;
        self.update();
    }

    pub fn update(&self) {
        for d in self.dependents.lock().unwrap().iter() {
            d.call();
        }
    }

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
    pub fn new<F: Fn() + Send + Sync + 'static>(f: F) -> Self {
        Self(Arc::new(Mutex::new(f)))
    }

    pub fn call(&self) {
        (self.0.lock().unwrap())()
    }
}

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

pub trait HookDependency: Send + Sync {
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
    pub fn mount(&self) {
        *self.0.lock().unwrap() = true;
        for hook_effect in self.1.lock().unwrap().iter() {
            hook_effect.call();
        }
        self.1.lock().unwrap().clear();
    }
}

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

pub fn use_mount() -> Mount {
    Mount(Arc::new(Mutex::new(true)), Arc::new(Mutex::new(Vec::new())))
}

pub fn use_mount_manual() -> Mount {
    Mount(
        Arc::new(Mutex::new(false)),
        Arc::new(Mutex::new(Vec::new())),
    )
}

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

pub fn use_sync_effect<
    T: Send + Sync + 'static,
    Ev: 'static,
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
                cx.emit_event(&encoder(&state));
            }
        },
        deps,
    );
}
