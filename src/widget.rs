use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{render_scope::RenderScope, state::DependencyHandler};

pub type BoxedElement = Box<dyn Element + Send + Sync>;
pub type BoxedComponent = Box<dyn Component + Send + Sync>;

pub trait Element: Send + Sync {
    #[allow(unused)]
    fn render(&mut self, scope: &mut RenderScope) {}
    #[allow(unused)]
    fn after_render(&mut self, scope: &RenderScope) {}
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait Component: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct Widget(
    pub Mutex<BoxedElement>,
    Mutex<HashMap<TypeId, BoxedComponent>>,
    Mutex<Box<dyn FnMut() -> WidgetLoad + Send + Sync>>,
    Mutex<Vec<Box<dyn DependencyHandler>>>,
);

impl Widget {
    pub fn new<F: FnMut() -> WidgetLoad + 'static + Send + Sync>(mut e: F) -> Self {
        let wl = e();
        Self(
            Mutex::new(wl.0),
            Mutex::new(wl.1),
            Mutex::new(Box::new(e)),
            Mutex::new(Vec::new()),
        )
    }

    pub fn refresh(self: &Arc<Self>) {
        let w = (self.2.lock().unwrap())();
        *self.0.lock().unwrap() = w.0;
        *self.1.lock().unwrap() = w.1;
    }

    pub fn auto_refresh(self: &Arc<Self>) {
        for d in self.3.lock().unwrap().iter() {
            if d.check() {
                self.refresh();
            }
        }
    }

    pub fn dependency<D: DependencyHandler + 'static>(self: &Arc<Self>, d: D) -> &Arc<Self> {
        d.add();
        self.3.lock().unwrap().push(Box::new(d));
        self
    }

    pub fn dependency_box(self: &Arc<Self>, d: Box<dyn DependencyHandler>) -> &Arc<Self> {
        d.add();
        self.3.lock().unwrap().push(d);
        self
    }

    pub fn component<C: Component + 'static>(self: &Arc<Self>, c: C) -> &Arc<Self> {
        self.1
            .lock()
            .unwrap()
            .entry(c.type_id())
            .or_insert_with(|| Box::new(c));
        self
    }

    pub fn set_component<C: Component + 'static>(self: &Arc<Self>, c: C) -> &Arc<Self> {
        self.1.lock().unwrap().insert(c.type_id(), Box::new(c));
        self
    }

    pub fn get<C: Component + 'static + Clone>(&self) -> Option<C> {
        self.1
            .lock()
            .unwrap()
            .get(&TypeId::of::<C>())
            .and_then(|c| c.as_any().downcast_ref::<C>())
            .map(|c| c.clone())
    }
}

pub struct WidgetLoad(BoxedElement, HashMap<TypeId, BoxedComponent>);

impl WidgetLoad {
    pub fn new<E: Element + 'static>(e: E) -> Self {
        Self(Box::new(e), HashMap::new())
    }

    pub fn component<C: Component + 'static>(mut self, c: C) -> Self {
        self.1.entry(c.type_id()).or_insert_with(|| Box::new(c));
        self
    }

    pub fn set_component<C: Component + 'static>(mut self, c: C) -> Self {
        self.1.insert(c.type_id(), Box::new(c));
        self
    }

    pub fn get<C: Component + 'static + Clone>(&self) -> Option<C> {
        self.1
            .get(&TypeId::of::<C>())
            .and_then(|c| c.as_any().downcast_ref::<C>())
            .map(|c| c.clone())
    }
}
