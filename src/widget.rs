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

use crate::{
    prelude::{Event, Handler},
    render_scope::RenderScope,
    state::DependencyHandler,
};

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
    fn render(
        &mut self,
        scope: &mut RenderScope,
        render_context: &crate::render_scope::RenderContext,
    ) {
    }

    /// Called after rendering, for follow-up logic or cleanup.
    #[allow(unused)]
    fn after_render(
        &mut self,
        scope: &mut RenderScope,
        render_context: &crate::render_scope::RenderContext,
    ) {
    }

    /// Called to draw child widgets, if any.
    #[allow(unused)]
    fn draw_child(&mut self, element: &Arc<Widget>) {}

    /// Called to undraw child widgets, if any.
    #[allow(unused)]
    fn undraw_child(&mut self, element: &Arc<Widget>) {}

    #[allow(unused)]
    fn event(&mut self, event: &dyn Event) {}

    fn is_ghost(&mut self) -> bool {
        false
    }

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
pub struct StaticWidget {
    element: Mutex<BoxedElement>,
    components: Mutex<HashMap<TypeId, BoxedComponent>>,
    focused: Mutex<bool>,
}

/// A widget with dynamic content and dependency tracking.
///
/// This widget supports reactive updates and can be rebuilt using
/// a provided `FnMut()` function when dependencies change.
pub struct DynWidget {
    element: Mutex<BoxedElement>,
    components: Mutex<HashMap<TypeId, BoxedComponent>>,
    load: Mutex<Box<dyn FnMut() -> WidgetLoad + Send + Sync>>,
    dependencies: Mutex<Vec<Box<dyn DependencyHandler>>>,
    injection: Mutex<Option<Box<dyn FnMut(WidgetLoad) -> WidgetLoad + Send + Sync>>>,
    focused: Mutex<bool>,
}

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
        Self::Static(StaticWidget::new(e))
    }

    pub fn new_dyn<F: FnMut() -> WidgetLoad + 'static + Send + Sync>(e: F) -> Self {
        Self::Dynamic(DynWidget::new(e))
    }
}

impl Widget {
    pub fn is_focused(&self) -> bool {
        match self {
            Self::Dynamic(w) => *w.focused.lock().unwrap(),
            Self::Static(w) => *w.focused.lock().unwrap(),
        }
    }

    pub fn is_ghost(&self) -> bool {
        match self {
            Self::Dynamic(w) => w.element.lock().unwrap().is_ghost(),
            Self::Static(w) => w.element.lock().unwrap().is_ghost(),
        }
    }

    pub fn set_focused(&self, f: bool) {
        match self {
            Self::Dynamic(w) => *w.focused.lock().unwrap() = f,
            Self::Static(w) => *w.focused.lock().unwrap() = f,
        }
    }

    pub fn get_elem(&'_ self) -> MutexGuard<'_, BoxedElement> {
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
                w.components.lock().unwrap().extend(f(wl).1);
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

    pub fn event<E: Event + Clone + 'static>(self: &Arc<Self>, e: &E) {
        if let Some(wrapper) = self.get::<Handler<E>>() {
            wrapper.call(self, e);
        }

        if self.is_focused() {
            match &**self {
                Widget::Dynamic(w) => {
                    w.get_elem().event(e);
                }
                Widget::Static(w) => {
                    w.get_elem().event(e);
                }
            }
        }
    }
}

impl StaticWidget {
    fn after_render(&self) {}
    fn get_elem(&'_ self) -> MutexGuard<'_, BoxedElement> {
        self.element.lock().unwrap()
    }
}

impl StaticWidget {
    pub fn new(e: BoxedElement) -> Self {
        Self {
            element: Mutex::new(e),
            components: Mutex::new(HashMap::new()),
            focused: Mutex::new(false),
        }
    }

    pub fn component<C: Component + 'static>(&self, c: C) {
        self.components
            .lock()
            .unwrap()
            .entry(c.type_id())
            .or_insert_with(|| Box::new(c));
    }

    pub fn set_component<C: Component + 'static>(&self, c: C) {
        self.components
            .lock()
            .unwrap()
            .insert(c.type_id(), Box::new(c));
    }

    pub fn get<C: Component + 'static + Clone>(&self) -> Option<C> {
        self.components
            .lock()
            .unwrap()
            .get(&TypeId::of::<C>())
            .and_then(|c| c.as_any().downcast_ref::<C>())
            .map(Clone::clone)
    }
}

impl DynWidget {
    fn after_render(&self) {}
    fn get_elem(&'_ self) -> MutexGuard<'_, BoxedElement> {
        self.element.lock().unwrap()
    }
}

impl DynWidget {
    pub fn new<F: FnMut() -> WidgetLoad + 'static + Send + Sync>(mut e: F) -> Self {
        let wl = e();
        Self {
            element: Mutex::new(wl.0),
            components: Mutex::new(wl.1),
            load: Mutex::new(Box::new(e)),
            dependencies: Mutex::new(Vec::new()),
            injection: Mutex::new(None),
            focused: Mutex::new(false),
        }
    }

    /// Replace or modify the widget's structure on reload and init.
    pub fn inject<F: FnMut(WidgetLoad) -> WidgetLoad + 'static + Send + Sync>(&self, f: F) {
        *self.injection.lock().unwrap() = Some(Box::new(f));
        self.refresh();
    }

    /// Rebuild the widget's content by re-evaluating the original function.
    pub fn refresh(&self) {
        let mut w = (self.load.lock().unwrap())();

        if let Some(we) = &mut *self.injection.lock().unwrap() {
            w = (we)(w);
        }

        *self.element.lock().unwrap() = w.0;
        *self.components.lock().unwrap() = w.1;
    }

    /// Re-evaluates the widget if any dependency has changed.
    pub fn auto_refresh(&self) {
        for d in self.dependencies.lock().unwrap().iter() {
            if d.check() {
                self.refresh();
            }
        }
    }

    /// Adds a dependency to this widget.
    pub fn dependency<D: DependencyHandler + 'static>(&self, d: D) {
        d.add();
        self.dependencies.lock().unwrap().push(Box::new(d));
    }

    /// Adds a boxed dependency.
    pub fn dependency_box(&self, d: Box<dyn DependencyHandler>) {
        d.add();
        self.dependencies.lock().unwrap().push(d);
    }

    pub fn component<C: Component + 'static>(&self, c: C) {
        self.components
            .lock()
            .unwrap()
            .entry(c.type_id())
            .or_insert_with(|| Box::new(c));
    }

    pub fn set_component<C: Component + 'static>(&self, c: C) {
        self.components
            .lock()
            .unwrap()
            .insert(c.type_id(), Box::new(c));
    }

    pub fn get<C: Component + 'static + Clone>(&self) -> Option<C> {
        self.components
            .lock()
            .unwrap()
            .get(&TypeId::of::<C>())
            .and_then(|c| c.as_any().downcast_ref::<C>())
            .map(Clone::clone)
    }
}
