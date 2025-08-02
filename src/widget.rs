//! Core widget infrastructure for OSUI.
//!
//! This module defines the traits and types that power the widget system, including:
//! - `Element` and `Component`: building blocks for renderable and state-carrying objects
//! - `Widget`: container type for wrapping elements and components
//! - `StaticWidget` and `DynWidget`: concrete widget implementations
//! - Dependency tracking and reactive updates for dynamic widgets

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
};

use crate::{render_scope::RenderScope, state::DependencyHandler};

/// A trait object for any renderable UI element.
pub type BoxedElement = Box<dyn Element + Send + Sync>;

/// A trait object for any component attached to a widget.
pub type BoxedComponent = Box<dyn Component + Send + Sync>;

/// Core trait for anything that can be rendered in the UI.
///
/// Elements are responsible for their own rendering logic and can define hooks
/// for lifecycle events and child rendering.
pub trait Element: Send + Sync {
    /// Called to perform rendering for the element.
    #[allow(unused)]
    fn render(&mut self, scope: &mut RenderScope) {}

    /// Called after rendering, for follow-up logic or cleanup.
    #[allow(unused)]
    fn after_render(&mut self, scope: &mut RenderScope) {}

    /// Called to draw child widgets, if any.
    #[allow(unused)]
    fn draw_child(&self, element: &Arc<Widget>) {}

    /// Returns a type-erased reference to this object.
    fn as_any(&self) -> &dyn Any;

    /// Returns a mutable type-erased reference to this object.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Optional trait for state or metadata attached to widgets.
///
/// Components can be used to store data such as layout style,
/// animation state, bindings, or other logic.
pub trait Component: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Container for a widget during initial construction.
///
/// Holds the root element and any associated components.
pub struct WidgetLoad(BoxedElement, HashMap<TypeId, BoxedComponent>);

/// A widget with fixed content and no dynamic behavior.
pub struct StaticWidget(Mutex<BoxedElement>, Mutex<HashMap<TypeId, BoxedComponent>>);

/// A widget with dynamic content and dependency tracking.
///
/// This widget supports reactive updates and can be rebuilt using
/// a provided `FnMut()` function when dependencies change.
pub struct DynWidget(
    Mutex<BoxedElement>,
    Mutex<HashMap<TypeId, BoxedComponent>>,
    Mutex<Box<dyn FnMut() -> WidgetLoad + Send + Sync>>,
    Mutex<Vec<Box<dyn DependencyHandler>>>,
    Mutex<Option<Box<dyn FnMut(WidgetLoad) -> WidgetLoad + Send + Sync>>>,
);

/// A reference-counted wrapper around either a static or dynamic widget.
///
/// Use `Arc<Widget>` as the standard way to store and pass around widgets in the UI tree.
pub enum Widget {
    Static(StaticWidget),
    Dynamic(DynWidget),
}

impl WidgetLoad {
    /// Creates a new `WidgetLoad` with a given root element.
    pub fn new<E: Element + 'static>(e: E) -> Self {
        Self(Box::new(e), HashMap::new())
    }

    /// Attaches a component if one of its type doesn't already exist.
    pub fn component<C: Component + 'static>(mut self, c: C) -> Self {
        self.1.entry(c.type_id()).or_insert_with(|| Box::new(c));
        self
    }

    /// Replaces any existing component of the same type.
    pub fn set_component<C: Component + 'static>(mut self, c: C) -> Self {
        self.1.insert(c.type_id(), Box::new(c));
        self
    }

    /// Attempts to retrieve a component of the given type.
    pub fn get<C: Component + 'static + Clone>(&self) -> Option<C> {
        self.1
            .get(&TypeId::of::<C>())
            .and_then(|c| c.as_any().downcast_ref::<C>())
            .map(|c| c.clone())
    }
}

impl Widget {
    pub fn new_static(e: BoxedElement) -> Self {
        Self::Static(StaticWidget(Mutex::new(e), Mutex::new(HashMap::new())))
    }

    pub fn new_dyn<F: FnMut() -> WidgetLoad + 'static + Send + Sync>(mut e: F) -> Self {
        let wl = e();
        Self::Dynamic(DynWidget(
            Mutex::new(wl.0),
            Mutex::new(wl.1),
            Mutex::new(Box::new(e)),
            Mutex::new(Vec::new()),
            Mutex::new(None),
        ))
    }
}

impl Widget {
    pub fn get_elem(&self) -> MutexGuard<BoxedElement> {
        match self {
            Widget::Static(w) => w.get_elem(),
            Widget::Dynamic(w) => w.get_elem(),
        }
    }

    pub fn after_render(&self) {
        match self {
            Widget::Static(w) => w.after_render(),
            Widget::Dynamic(w) => w.after_render(),
        }
    }

    pub fn component<C: Component + 'static>(self: &Arc<Self>, c: C) -> &Arc<Self> {
        match &**self {
            Widget::Static(w) => {
                w.component(c);
            }
            Widget::Dynamic(w) => {
                w.component(c);
            }
        }
        self
    }

    pub fn set_component<C: Component + 'static>(self: &Arc<Self>, c: C) -> &Arc<Self> {
        match &**self {
            Widget::Static(w) => {
                w.set_component(c);
            }
            Widget::Dynamic(w) => {
                w.set_component(c);
            }
        }
        self
    }

    pub fn get<C: Component + 'static + Clone>(&self) -> Option<C> {
        match self {
            Widget::Static(w) => w.get(),
            Widget::Dynamic(w) => w.get(),
        }
    }

    pub fn inject<F: FnMut(WidgetLoad) -> WidgetLoad + 'static + Send + Sync>(
        self: &Arc<Self>,
        mut f: F,
    ) {
        match &**self {
            Widget::Dynamic(w) => {
                w.inject(f);
            }
            Widget::Static(w) => {
                let wl = WidgetLoad::new(String::new());
                w.1.lock().unwrap().extend(f(wl).1);
            }
        }
    }

    pub fn refresh(self: &Arc<Self>) {
        match &**self {
            Widget::Dynamic(w) => {
                w.refresh();
            }
            Widget::Static(_) => {}
        }
    }

    pub fn auto_refresh(self: &Arc<Self>) {
        match &**self {
            Widget::Dynamic(w) => {
                w.auto_refresh();
            }
            Widget::Static(_) => {}
        }
    }

    pub fn dependency<D: DependencyHandler + 'static>(self: &Arc<Self>, d: D) -> &Arc<Self> {
        match &**self {
            Widget::Dynamic(w) => {
                w.dependency(d);
            }
            Widget::Static(_) => {}
        }
        self
    }

    pub fn dependency_box(self: &Arc<Self>, d: Box<dyn DependencyHandler>) -> &Arc<Self> {
        match &**self {
            Widget::Dynamic(w) => {
                w.dependency_box(d);
            }
            Widget::Static(_) => {}
        }
        self
    }
}

impl StaticWidget {
    fn after_render(&self) {}
    fn get_elem(&self) -> MutexGuard<BoxedElement> {
        self.0.lock().unwrap()
    }
}

impl StaticWidget {
    pub fn new(e: BoxedElement) -> Self {
        Self(Mutex::new(e), Mutex::new(HashMap::new()))
    }

    pub fn component<C: Component + 'static>(&self, c: C) {
        self.1
            .lock()
            .unwrap()
            .entry(c.type_id())
            .or_insert_with(|| Box::new(c));
    }

    pub fn set_component<C: Component + 'static>(&self, c: C) {
        self.1.lock().unwrap().insert(c.type_id(), Box::new(c));
    }

    pub fn get<C: Component + 'static + Clone>(&self) -> Option<C> {
        self.1
            .lock()
            .unwrap()
            .get(&TypeId::of::<C>())
            .and_then(|c| c.as_any().downcast_ref::<C>())
            .map(Clone::clone)
    }
}

impl DynWidget {
    fn after_render(&self) {}
    fn get_elem(&self) -> MutexGuard<BoxedElement> {
        self.0.lock().unwrap()
    }
}

impl DynWidget {
    pub fn new<F: FnMut() -> WidgetLoad + 'static + Send + Sync>(mut e: F) -> Self {
        let wl = e();
        Self(
            Mutex::new(wl.0),
            Mutex::new(wl.1),
            Mutex::new(Box::new(e)),
            Mutex::new(Vec::new()),
            Mutex::new(None),
        )
    }

    /// Replace or modify the widget's structure on reload and init.
    pub fn inject<F: FnMut(WidgetLoad) -> WidgetLoad + 'static + Send + Sync>(&self, f: F) {
        *self.4.lock().unwrap() = Some(Box::new(f));
        self.refresh();
    }

    /// Rebuild the widget's content by re-evaluating the original function.
    pub fn refresh(&self) {
        let mut w = (self.2.lock().unwrap())();

        if let Some(we) = &mut *self.4.lock().unwrap() {
            w = (we)(w);
        }

        *self.0.lock().unwrap() = w.0;
        *self.1.lock().unwrap() = w.1;
    }

    /// Re-evaluates the widget if any dependency has changed.
    pub fn auto_refresh(&self) {
        for d in self.3.lock().unwrap().iter() {
            if d.check() {
                self.refresh();
            }
        }
    }

    /// Adds a dependency to this widget.
    pub fn dependency<D: DependencyHandler + 'static>(&self, d: D) {
        d.add();
        self.3.lock().unwrap().push(Box::new(d));
    }

    /// Adds a boxed dependency.
    pub fn dependency_box(&self, d: Box<dyn DependencyHandler>) {
        d.add();
        self.3.lock().unwrap().push(d);
    }

    pub fn component<C: Component + 'static>(&self, c: C) {
        self.1
            .lock()
            .unwrap()
            .entry(c.type_id())
            .or_insert_with(|| Box::new(c));
    }

    pub fn set_component<C: Component + 'static>(&self, c: C) {
        self.1.lock().unwrap().insert(c.type_id(), Box::new(c));
    }

    pub fn get<C: Component + 'static + Clone>(&self) -> Option<C> {
        self.1
            .lock()
            .unwrap()
            .get(&TypeId::of::<C>())
            .and_then(|c| c.as_any().downcast_ref::<C>())
            .map(Clone::clone)
    }
}
