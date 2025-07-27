use std::ops::{Deref, DerefMut};

use crate::dependency::DependencyHandler;

#[derive(Debug)]
pub struct State<T> {
    pub value: *mut T,
    dependencies: *mut usize,
    changed: *mut usize, // set to dependencies when the value is changed
}

pub fn use_state<T>(mut v: T) -> State<T> {
    State {
        value: &mut v,
        dependencies: &mut 0,
        changed: &mut 0,
    }
}

impl<T> State<T> {
    pub fn set(&self, v: T) {
        unsafe {
            *self.value = v;
        }

        self.change();
    }

    pub fn change(&self) {
        unsafe {
            *self.changed = *self.dependencies;
        }
    }
}

impl<T: std::fmt::Debug> DependencyHandler for State<T> {
    fn check(&self) -> bool {
        unsafe {
            let i = *self.changed > 0;
            if i {
                *self.changed -= 1;
            }
            i
        }
    }

    fn add(&self) {
        unsafe {
            *self.dependencies += 1;
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for State<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", unsafe { &*self.value })
    }
}

unsafe impl<T> Send for State<T> {}
unsafe impl<T> Sync for State<T> {}

impl<T> Deref for State<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.value }
    }
}

impl<T> DerefMut for State<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            self.change();

            &mut *self.value
        }
    }
}

impl<T> Clone for State<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value,
            dependencies: self.dependencies,
            changed: self.changed,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.changed = source.changed;
        self.dependencies = source.dependencies;
        self.value = source.value;
    }
}
