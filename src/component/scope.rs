//! # Scope Module
//!
//! Provides the Scope type for managing component hierarchies.
//! Scopes group child components and manage their lifecycle.

use std::sync::{Arc, Mutex};

use crate::{engine::CommandExecutor, View, ViewWrapper};

use super::{context::Context, ComponentImpl};

/// A scope groups child components and manages their rendering
///
/// Scopes form the hierarchical structure of a component tree.
/// Each scope contains references to its child components and their
/// optional view wrappers (for layout/styling).
pub struct Scope {
    /// Child components with optional view wrappers
    pub children: Mutex<Vec<(Arc<Context>, Option<ViewWrapper>)>>,
    /// Command executor for this scope's children
    executor: Arc<dyn CommandExecutor>,
}

impl Scope {
    /// Creates a new scope with the given command executor
    pub fn new(executor: Arc<dyn CommandExecutor>) -> Arc<Self> {
        Arc::new(Self {
            children: Mutex::new(Vec::new()),
            executor,
        })
    }

    /// Adds a child component to this scope
    ///
    /// The view_wrapper is optional and can be used for layout or styling.
    pub fn child<F: ComponentImpl + 'static>(
        self: &Arc<Self>,
        child: F,
        view_wrapper: Option<ViewWrapper>,
    ) {
        let ctx = Context::new(child, self.executor.clone());

        ctx.refresh();

        self.children.lock().unwrap().push((ctx, view_wrapper));
    }

    /// Adds a view directly to this scope
    pub fn view(self: &Arc<Self>, view: View) {
        let ctx = Context::new(view, self.executor.clone());

        ctx.refresh();

        self.children.lock().unwrap().push((ctx, None));
    }

    /// Adds a view directly to this scope
    pub fn view_wrapped(self: &Arc<Self>, view: View, view_wrapper: ViewWrapper) {
        let ctx = Context::new(view, self.executor.clone());

        ctx.refresh();

        self.children.lock().unwrap().push((ctx, Some(view_wrapper)));
    }
}
