use std::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex, MutexGuard},
};

/// Trait for tracking dependencies and reactivity.
pub trait DependencyHandler: std::fmt::Debug + Send + Sync {
    /// Called when a dependent is registered.
    fn add(&self);
    /// Returns `true` if the state has changed since the last check.
    fn check(&self) -> bool;
}

#[derive(Debug, Clone)]
pub struct State<T> {
    inner: Arc<Mutex<Inner<T>>>,
}

#[derive(Debug)]
pub struct Inner<T> {
    value: T,
    dependencies: usize,
    changed: usize,
}

pub fn use_state<T>(v: T) -> State<T> {
    State {
        inner: Arc::new(Mutex::new(Inner {
            value: v,
            dependencies: 0,
            changed: 0,
        })),
    }
}

impl<T: Clone> State<T> {
    /// Gets the cloned value, recommended for preventing deadlocks
    pub fn get_dl(&self) -> T {
        self.inner.lock().unwrap().value.clone()
    }
}

impl<T> State<T> {
    /// Gets a lock on the state for read/write access.
    pub fn get(&self) -> MutexGuard<'_, Inner<T>> {
        self.inner.lock().unwrap()
    }

    /// Sets the value and marks it as changed.
    pub fn set(&self, v: T) {
        let mut inner = self.inner.lock().unwrap();
        inner.value = v;
        inner.changed = inner.dependencies;
    }

    /// Marks the state as updated.
    pub fn update(&self) {
        let mut inner = self.inner.lock().unwrap();
        inner.changed = inner.dependencies;
    }
}

impl<T: Debug + Send + Sync> DependencyHandler for State<T> {
    fn check(&self) -> bool {
        let mut inner = self.inner.lock().unwrap();
        let i = inner.changed > 0;
        if i {
            inner.changed -= 1;
        }
        i
    }

    fn add(&self) {
        let mut inner = self.inner.lock().unwrap();
        inner.dependencies += 1;
    }
}

impl<T: Display> Display for State<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let inner = self.inner.lock().unwrap();
        write!(f, "{}", inner.value)
    }
}

impl<T> Deref for Inner<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Inner<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.changed = self.dependencies;
        &mut self.value
    }
}