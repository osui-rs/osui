use std::sync::{Arc, Mutex};

pub struct StateManager(fn(Arc<StateManager>));

pub struct State<S>(Arc<StateManager>, Arc<Mutex<S>>);

impl StateManager {
    pub fn new(recall: fn(Arc<StateManager>)) -> Arc<StateManager> {
        Arc::new(StateManager(recall))
    }

    pub fn use_state<S>(self: Arc<Self>, state: S) -> State<S> {
        State(self.clone(), Arc::new(Mutex::new(state)))
    }
}

impl<S> State<S> {
    pub fn set(&self, to: S) {
        *self.1.lock().unwrap() = to;
        (self.0 .0)(self.0.clone())
    }
}

impl State<i32> {
    pub fn get(&self) -> i32 {
        *self.1.lock().unwrap()
    }
}
