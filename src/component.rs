use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::Node;

pub trait Event {
    fn as_any(&self) -> &dyn Any;
}

impl<'a> dyn Event + 'a {
    pub fn get<T: Event + 'static>(&self) -> Option<&T> {
        self.as_any().downcast_ref()
    }
}

pub type Component = Arc<dyn Fn(&Arc<Context>) -> Vec<Node> + Send + Sync>;
pub type EventHandler = Arc<Mutex<dyn FnMut(&Arc<Context>, &dyn Event) + Send + Sync>>;

pub struct Context {
    component: Mutex<Component>,
    nodes: Mutex<Vec<Node>>,
    event_handlers: Mutex<HashMap<TypeId, Vec<EventHandler>>>,
}

impl Context {
    pub fn new<F: Fn(&Arc<Context>) -> Vec<Node> + Send + Sync + 'static>(
        component: F,
    ) -> Arc<Self> {
        Arc::new(Self {
            component: Mutex::new(Arc::new(component)),
            nodes: Mutex::new(Vec::new()),
            event_handlers: Mutex::new(HashMap::new()),
        })
    }

    pub fn refresh(self: &Arc<Context>) {
        let c = self.component.lock().unwrap().clone();
        *self.nodes.lock().unwrap() = (c)(self);
    }

    pub fn get_nodes(self: &Arc<Context>) -> Vec<Node> {
        self.nodes.lock().unwrap().clone()
    }

    pub fn on_event<T: Event + 'static, F: Fn(&Arc<Context>, &T) + Send + Sync + 'static>(
        self: &Arc<Context>,
        handler: F,
    ) {
        self.event_handlers
            .lock()
            .unwrap()
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Vec::new())
            .push(Arc::new(Mutex::new(
                move |ctx: &Arc<Context>, event: &dyn Event| {
                    if let Some(e) = event.get::<T>() {
                        (handler)(ctx, e);
                    }
                },
            )));
    }

    pub fn emit_event<E: Event + 'static>(self: &Arc<Context>, event: &E) {
        if let Some(v) = self.event_handlers.lock().unwrap().get(&TypeId::of::<E>()) {
            for i in v {
                (i.lock().unwrap())(self, event);
            }
        }
    }

    pub fn emit_event_threaded<E: Event + Send + Sync + Clone + 'static>(
        self: &Arc<Context>,
        event: &E,
    ) {
        if let Some(v) = self.event_handlers.lock().unwrap().get(&TypeId::of::<E>()) {
            for i in v {
                let i = i.clone();
                let event = event.clone();
                let s = self.clone();
                std::thread::spawn(move || {
                    (i.lock().unwrap())(&s, &event);
                });
            }
        }
    }
}
