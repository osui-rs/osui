use std::sync::Arc;

use crate::{widget::Element};

pub trait DependencyHandler: std::fmt::Debug + Send + Sync {
    fn add(&self);
    fn check(&self) -> bool;
}

pub struct Dependency<T: Element> {
    handlers: Vec<std::sync::Arc<dyn DependencyHandler>>,
    element: T,
    element_reset: Box<dyn FnMut() -> T + Send + Sync>,
}

impl<T: Element> Dependency<T> {
    pub fn new<F: FnMut() -> T + Send + Sync + 'static>(mut f: F) -> Self {
        Self {
            handlers: Vec::new(),
            element: f(),
            element_reset: Box::new(f),
        }
    }

    pub fn auto_reset(&mut self) {
        for i in &self.handlers {
            if i.check() {
                self.element = (self.element_reset)();
            }
        }
    }

    pub fn add<D: DependencyHandler + 'static>(mut self, dep: D) -> Self {
        dep.add();
        self.handlers.push(Arc::new(dep));
        self
    }
}

impl<T: Element + 'static> Element for Dependency<T> {
    fn render(&mut self, scope: &mut crate::render_scope::RenderScope) {
        self.element.render(scope);
        self.auto_reset();
    }

    fn after_render(&mut self, scope: &crate::render_scope::RenderScope) {
        self.element.after_render(scope);
        self.auto_reset();
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
