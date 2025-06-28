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

pub struct EventManager {
    handlers:
        Mutex<HashMap<TypeId, Vec<Arc<Mutex<dyn FnMut(&Arc<EventManager>, Box<dyn Event>)>>>>>,
    states: Mutex<Option<Arc<StateManager>>>,
}

impl EventManager {
    pub fn new() -> Arc<EventManager> {
        Arc::new(EventManager {
            handlers: Mutex::new(HashMap::new()),
            states: Mutex::new(None),
        })
    }

    pub fn on<E: Event + 'static, F: FnMut(&Arc<EventManager>, Box<E>) + 'static>(
        self: &Arc<Self>,
        mut f: F,
    ) {
        let states = self.states.lock().unwrap().clone();
        self.handlers
            .lock()
            .unwrap()
            .entry(TypeId::of::<E>())
            .or_insert_with(Vec::new)
            .push(Arc::new(Mutex::new(
                move |e: &Arc<EventManager>, evt: Box<dyn Event>| {
                    if let Ok(concrete) = evt.as_any().downcast::<E>() {
                        f(&e, concrete);
                        if let Some(states) = &states {
                            states.clone().flush();
                        }
                    }
                },
            )));
    }

    pub fn dispatch<E: Event + 'static + Clone>(self: &Arc<Self>, evt: E) {
        let tid = evt.type_id();

        if let Some(handlers) = self.handlers.lock().unwrap().get_mut(&tid) {
            // Clone handlers to avoid borrowing self during call
            let handler_clones: Vec<_> = handlers.iter().cloned().collect();

            for h in handler_clones {
                let mut ha = h.lock().unwrap();
                (ha)(self, Box::new(evt.clone()));
            }
        }

        if tid == TypeId::of::<Close>() {
            std::process::exit(0);
        }
    }

    pub fn set_state_manager(self: &Arc<Self>, states: Arc<StateManager>) {
        *self.states.lock().unwrap() = Some(states);
    }
}
