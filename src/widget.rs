use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
};

use crate::{render_scope::RenderScope, state::DependencyHandler};

pub type BoxedElement = Box<dyn Element + Send + Sync>;
pub type BoxedComponent = Box<dyn Component + Send + Sync>;

pub trait Element: Send + Sync {
    #[allow(unused)]
    fn render(&mut self, scope: &mut RenderScope) {}
    #[allow(unused)]
    fn after_render(&mut self, scope: &mut RenderScope) {}
    #[allow(unused)]
    fn draw_child(&self, element: &Arc<Widget>) {}
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait Component: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
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

pub struct StaticWidget(Mutex<BoxedElement>, Mutex<HashMap<TypeId, BoxedComponent>>);

pub struct DynWidget(
    Mutex<BoxedElement>,
    Mutex<HashMap<TypeId, BoxedComponent>>,
    Mutex<Box<dyn FnMut() -> WidgetLoad + Send + Sync>>,
    Mutex<Vec<Box<dyn DependencyHandler>>>,
    Mutex<Option<Box<dyn FnMut(WidgetLoad) -> WidgetLoad + Send + Sync>>>,
);

pub enum Widget {
    Static(StaticWidget),
    Dynamic(DynWidget),
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
        f: F,
    ) {
        match &**self {
            Widget::Dynamic(w) => {
                w.inject(f);
            }
            Widget::Static(_) => {}
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

    pub fn inject<F: FnMut(WidgetLoad) -> WidgetLoad + 'static + Send + Sync>(&self, f: F) {
        *self.4.lock().unwrap() = Some(Box::new(f));
        self.refresh();
    }

    pub fn refresh(&self) {
        let mut w = (self.2.lock().unwrap())();

        if let Some(we) = &mut *self.4.lock().unwrap() {
            w = (we)(w);
        }

        *self.0.lock().unwrap() = w.0;
        *self.1.lock().unwrap() = w.1;
    }

    pub fn auto_refresh(&self) {
        for d in self.3.lock().unwrap().iter() {
            if d.check() {
                self.refresh();
            }
        }
    }

    pub fn dependency<D: DependencyHandler + 'static>(&self, d: D) {
        d.add();
        self.3.lock().unwrap().push(Box::new(d));
    }

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
