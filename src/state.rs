use std::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex, MutexGuard},
};

#[derive(Clone)]
pub struct HookEffect(Arc<Mutex<dyn FnMut() + Send + Sync>>);

#[derive(Debug, Clone)]
pub struct State<T> {
    value: Arc<Mutex<T>>,
    dependents: Arc<Mutex<Vec<HookEffect>>>,
}

pub struct Inner<'a, T> {
    value: MutexGuard<'a, T>,
    dependents: Arc<Mutex<Vec<HookEffect>>>,
    updated: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct Mount;

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

pub trait HookDependency {
    fn on_update(&self, hook: HookEffect);
}

impl<T> HookDependency for State<T> {
    fn on_update(&self, hook: HookEffect) {
        self.dependents.lock().unwrap().push(hook);
    }
}

impl HookDependency for Mount {
    fn on_update(&self, hook: HookEffect) {
        hook.call();
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
    Mount
}
