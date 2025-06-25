use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::Close;

pub trait Event {
    fn as_any(self: Box<Self>) -> Box<dyn Any>;
}

pub struct EventHandlerFn(pub Box<dyn FnMut(Box<dyn Event>)>);

impl std::fmt::Debug for EventHandlerFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FnMut()")
    }
}

#[derive(Debug)]
pub struct EventManager {
    handlers: HashMap<TypeId, Vec<EventHandlerFn>>,
}

impl EventManager {
    pub fn new() -> EventManager {
        EventManager {
            handlers: HashMap::new(),
        }
    }

    pub fn on<E: Event + 'static, F: FnMut(Box<E>) + 'static>(&mut self, mut f: F) {
        let wrapper = Box::new(move |evt: Box<dyn Event>| {
            if let Ok(concrete) = evt.as_any().downcast::<E>() {
                f(concrete);
            }
        }) as Box<dyn FnMut(Box<dyn Event>)>;

        self.handlers
            .entry(TypeId::of::<E>())
            .or_insert_with(Vec::new)
            .push(EventHandlerFn(wrapper));
    }

    pub fn dispatch<E: Event + 'static + Clone>(&mut self, evt: E) {
        let tid = evt.type_id();
        if let Some(handlers) = self.handlers.get_mut(&tid) {
            for h in handlers {
                (h.0)(Box::new(evt.clone()));
            }
        }

        if tid == TypeId::of::<Close>() {
            std::process::exit(0);
        }
    }
}
