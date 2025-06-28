use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{event, state::StateManager};

event!(Close);

pub trait Event {
    fn as_any(&self) -> &dyn Any;
}

pub struct EventManager {
    handlers: Mutex<HashMap<TypeId, Vec<Arc<Mutex<dyn FnMut(&Arc<EventManager>, &dyn Event)>>>>>,
    states: Mutex<Option<Arc<StateManager>>>,
    event_stack: Mutex<Vec<(TypeId, Box<dyn Event>)>>,
}

impl EventManager {
    pub fn new() -> Arc<EventManager> {
        Arc::new(EventManager {
            handlers: Mutex::new(HashMap::new()),
            states: Mutex::new(None),
            event_stack: Mutex::new(Vec::new()),
        })
    }

    pub fn on<E: Event + 'static, F: FnMut(&Arc<EventManager>, &E) + 'static>(
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
                move |e: &Arc<EventManager>, evt: &dyn Event| {
                    if let Some(concrete) = evt.as_any().downcast_ref::<E>() {
                        f(&e, concrete);
                        if let Some(states) = &states {
                            states.clone().flush();
                        }
                    }
                },
            )));
    }

    fn dispatch_single(self: &Arc<Self>, tid: TypeId, evt: &dyn Event) {
        if let Some(handlers) = self.handlers.lock().unwrap().get_mut(&tid) {
            let handler_clones: Vec<_> = handlers.iter().cloned().collect();

            for h in handler_clones {
                let mut ha = h.lock().unwrap();
                (ha)(self, evt);
            }
        }
    }

    pub fn dispatch<E: Event + 'static + Clone>(self: &Arc<Self>, evt: E) {
        self.event_stack
            .lock()
            .unwrap()
            .push((evt.type_id(), Box::new(evt)));

        while self.event_stack.lock().unwrap().len() > 0 {
            dis
        }
    }

    pub fn close(self: &Arc<Self>) {
        self.dispatch(Close);
        std::process::exit(0);
    }

    pub fn set_state_manager(self: &Arc<Self>, states: Arc<StateManager>) {
        *self.states.lock().unwrap() = Some(states);
    }
}
