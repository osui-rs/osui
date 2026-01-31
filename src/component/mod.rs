//! # Component Module
//!
//! Provides the component system that forms the foundation of OSUI.
//! Components are reusable units of UI that can manage their own state
//! and respond to events.

pub mod context;
pub mod scope;

use std::{
    any::Any,
    sync::{Arc, Mutex},
};

use crate::View;

use context::Context;

/// A Component is an implementor of the ComponentImpl trait, wrapped in Arc
pub type Component = Arc<dyn ComponentImpl>;

/// An event handler function stored in a mutex for thread-safe mutation
pub type EventHandler = Arc<Mutex<dyn FnMut(&Arc<Context>, &dyn Any) + Send + Sync>>;

/// Trait implemented by components to render themselves
pub trait ComponentImpl: Send + Sync {
    /// Renders the component within the given context, returning a View
    fn call(&self, cx: &Arc<Context>) -> View;
}

impl ComponentImpl for View {
    fn call(&self, _: &Arc<Context>) -> View {
        self.clone()
    }
}

impl<F> ComponentImpl for F
where
    F: Fn(&Arc<Context>) -> View + Send + Sync,
{
    fn call(&self, ctx: &Arc<Context>) -> View {
        self(ctx)
    }
}
