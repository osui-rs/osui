use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{event, state::StateManager};

event!(Close);

pub trait Event {
    fn as_any(self: Box<Self>) -> Box<dyn Any>;
}

#[derive(Clone)]
pub struct EventHandlerFn(pub Arc<Mutex<dyn FnMut(&mut EventManager, Box<dyn Event>)>>);

impl std::fmt::Debug for EventHandlerFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FnMut()")
    }
}

pub struct EventManager {
    handlers: HashMap<TypeId, Vec<EventHandlerFn>>,
    states: Option<Arc<StateManager>>,
}

impl EventManager {
    pub fn new() -> EventManager {
        EventManager {
            handlers: HashMap::new(),
            states: None,
        }
    }

    pub fn on<E: Event + 'static, F: FnMut(&mut EventManager, Box<E>) + 'static>(
        &mut self,
        mut f: F,
    ) {
        let states = self.states.clone();
        let wrapper = Arc::new(Mutex::new(
            move |e: &mut EventManager, evt: Box<dyn Event>| {
                if let Ok(concrete) = evt.as_any().downcast::<E>() {
                    f(e, concrete);
                    if let Some(states) = &states {
                        states.clone().flush();
                    }
                }
            },
        )) as Arc<Mutex<dyn FnMut(&mut EventManager, Box<dyn Event>)>>;

        self.handlers
            .entry(TypeId::of::<E>())
            .or_insert_with(Vec::new)
            .push(EventHandlerFn(wrapper));
    }

    pub fn dispatch<E: Event + 'static + Clone>(&mut self, evt: E) {
        let tid = evt.type_id();

        if let Some(handlers) = self.handlers.get_mut(&tid) {
            // Clone handlers to avoid borrowing self during call
            let handler_clones: Vec<_> = handlers.iter().cloned().collect();

            for h in handler_clones {
                let mut ha = h.0.lock().unwrap();
                (ha)(self, Box::new(evt.clone()));
            }
        }

        if tid == TypeId::of::<Close>() {
            std::process::exit(0);
        }
    }

    pub fn set_state_manager(&mut self, states: Arc<StateManager>) {
        self.states = Some(states);
    }
}
