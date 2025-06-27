use std::{
    any::Any,
    collections::HashMap,
    marker::PhantomData,
    sync::{Arc, Mutex},
};

/// ---------- public API ----------

pub struct StateManager {
    recall: fn(Arc<StateManager>),
    store: Mutex<HashMap<usize, Box<dyn Any + Send + Sync>>>,
    cursor: Mutex<usize>,
}

pub struct State<S> {
    manager: Arc<StateManager>,
    id: usize,
    _pd: PhantomData<S>,
}

impl StateManager {
    pub fn new(recall: fn(Arc<StateManager>)) -> Arc<Self> {
        Arc::new(Self {
            recall,
            store: Mutex::new(HashMap::new()),
            cursor: Mutex::new(0),
        })
    }

    /// Call this at the very top of every “render” before doing any `use_state` calls
    pub fn begin(&self) {
        *self.cursor.lock().unwrap() = 0;
    }

    pub fn use_state<S: 'static + Send + Sync>(self: &Arc<Self>, initial: S) -> State<S> {
        // `slot` == position of this call in the render
        let mut cur = self.cursor.lock().unwrap();
        let slot = *cur;
        *cur += 1;

        // Insert the initial value only the first time we see this slot
        let mut store = self.store.lock().unwrap();
        store
            .entry(slot)
            .or_insert_with(|| Box::new(Mutex::new(initial)));

        State {
            manager: Arc::clone(self),
            id: slot,
            _pd: PhantomData,
        }
    }
}

impl<S: 'static + Clone + Send + Sync> State<S> {
    pub fn get(&self) -> S {
        let store = self.manager.store.lock().unwrap();
        let x = store[&self.id]
            .downcast_ref::<Mutex<S>>()
            .unwrap()
            .lock()
            .unwrap()
            .clone();
        x
    }

    pub fn set(&self, val: S) {
        {
            let store = self.manager.store.lock().unwrap();
            *store[&self.id]
                .downcast_ref::<Mutex<S>>()
                .unwrap()
                .lock()
                .unwrap() = val;
        }
    }

    pub fn update(&self) {
        (self.manager.recall)(Arc::clone(&self.manager));
    }
}
