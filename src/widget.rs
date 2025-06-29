use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::render_scope::RenderScope;

pub trait Element {
    #[allow(unused)]
    fn render(&mut self, scope: &mut RenderScope) {}
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait Component {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct Widget(
    pub Mutex<Box<dyn Element>>,
    Mutex<HashMap<TypeId, Box<dyn Component>>>,
);

impl Widget {
    pub fn new(e: Box<dyn Element>) -> Self {
        Self(Mutex::new(e), Mutex::new(HashMap::new()))
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
