//! # Frontend Module
//!
//! Provides the RSX (React-like Syntax) system for composing components.
//! This module defines the structure for building component hierarchies
//! with static and dynamic scopes, similar to React's JSX.

use std::sync::Arc;

use crate::component::{context::Context, scope::Scope};
use crate::{render::Point, hooks::HookDependency, View};

/// Trait for converting values to RSX
pub trait ToRsx {
    /// Convert to RSX representation
    fn to_rsx(&self) -> Rsx;
}

/// Scope types for RSX components
#[derive(Clone)]
pub enum RsxScope {
    /// Static scope - executed once and never updated
    Static(Arc<dyn Fn(&Arc<Scope>) + Send + Sync>),
    /// Dynamic scope - re-executed when dependencies change
    Dynamic(
        Arc<dyn Fn(&Arc<Scope>) + Send + Sync>,
        Vec<Arc<dyn HookDependency>>,
    ),
    /// Child RSX scope for composition
    Child(Rsx),
}

/// RSX (React-like Syntax) builder for component hierarchies
/// 
/// Represents a collection of scopes that define component structure.
/// Scopes can be static (execute once) or dynamic (reactive to changes).
#[derive(Clone)]
pub struct Rsx(Vec<RsxScope>);

impl Rsx {
    /// Creates a new empty RSX
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Adds a static scope to this RSX
    /// 
    /// The provided function is executed once during rendering
    /// and will not be re-executed on dependency changes.
    pub fn static_scope<F: Fn(&Arc<Scope>) + Send + Sync + 'static>(&mut self, scope: F) {
        self.0.push(RsxScope::Static(Arc::new(scope)));
    }

    /// Adds a dynamic scope to this RSX
    /// 
    /// The provided function is executed when dependencies change,
    /// allowing for reactive updates similar to React hooks.
    pub fn dynamic_scope<F: Fn(&Arc<Scope>) + Send + Sync + 'static>(
        &mut self,
        drawer: F,
        dependencies: Vec<Arc<dyn HookDependency>>,
    ) {
        self.0
            .push(RsxScope::Dynamic(Arc::new(drawer), dependencies));
    }

    /// Adds a child RSX
    pub fn child<R: ToRsx>(&mut self, child: R) {
        self.0.push(RsxScope::Child(child.to_rsx()));
    }

    /// Generates child components within the given context
    pub fn generate_children(&self, context: &Arc<Context>) {
        let executor = context.get_executor();

        for scope in &self.0 {
            match scope {
                RsxScope::Static(scope_fn) => {
                    let scope = Scope::new(executor.clone());
                    (scope_fn)(&scope);
                    context.add_scope(scope)
                }
                RsxScope::Dynamic(drawer, dependencies) => {
                    let drawer = drawer.clone();
                    context.dyn_scope(
                        move |c| drawer(c),
                        &dependencies.iter().map(|d| d.as_ref()).collect::<Vec<_>>(),
                    );
                }
                RsxScope::Child(child) => child.generate_children(context),
            }
        }
    }

    /// Converts this RSX to a View
    pub fn view(&self, context: &Arc<Context>) -> View {
        let context = context.clone();

        self.generate_children(&context);

        Arc::new({
            move |ctx| {
                context.draw_children(ctx);
            }
        })
    }
}

impl ToRsx for &Rsx {
    fn to_rsx(&self) -> Rsx {
        Rsx(self.0.clone())
    }
}

impl<T: std::fmt::Display> ToRsx for T {
    fn to_rsx(&self) -> Rsx {
        let s = self.to_string();
        Rsx(vec![RsxScope::Static(Arc::new(move |scope| {
            let s = s.clone();
            scope.view(Arc::new(move |ctx| ctx.draw_text(Point { x: 0, y: 0 }, &s)))
        }))])
    }
}
