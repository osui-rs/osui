use std::{
    any::Any,
    collections::HashMap,
    fmt::Display,
    marker::PhantomData,
    ops::{Add, AddAssign, Div, Sub, SubAssign},
    sync::{Arc, Mutex},
};

/// ---------- public API ----------

pub struct StateManager {
    recall: Box<dyn Fn(&Arc<Self>)>,
    store: Mutex<HashMap<usize, Box<dyn Any + Send + Sync>>>,
    cursor: Mutex<usize>,
    dirty: Mutex<bool>,
}

pub struct State<S> {
    manager: Arc<StateManager>,
    id: usize,
    _pd: PhantomData<S>,
}

impl StateManager {
    pub fn new<F: Fn(&Arc<Self>) + 'static>(recall: F) -> Arc<Self> {
        Arc::new(Self {
            recall: Box::new(recall),
            store: Mutex::new(HashMap::new()),
            cursor: Mutex::new(0),
            dirty: Mutex::new(false),
        })
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

    pub fn flush(self: &Arc<Self>) {
        let mut d = self.dirty.lock().unwrap();
        if *d {
            *d = false;
            drop(d);

            *self.cursor.lock().unwrap() = 0;
            (self.recall)(self)
        }
    }
}

impl<S: 'static> State<S> {
    pub fn dirty(&self) {
        *self.manager.dirty.lock().unwrap() = true;
    }
}

impl<S: Display + 'static> Display for State<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.manager.store.lock().unwrap()[&self.id]
            .downcast_ref::<Mutex<S>>()
            .unwrap()
            .lock()
            .unwrap()
            .fmt(f)
    }
}

impl<S: Add + Clone + 'static> Add<S> for State<S> {
    type Output = <S as Add>::Output;

    fn add(self, rhs: S) -> Self::Output {
        self.manager.store.lock().unwrap()[&self.id]
            .downcast_ref::<Mutex<S>>()
            .unwrap()
            .lock()
            .unwrap()
            .clone()
            .add(rhs)
    }
}

impl<S: Sub + Clone + 'static> Sub<S> for State<S> {
    type Output = <S as Sub>::Output;

    fn sub(self, rhs: S) -> Self::Output {
        self.manager.store.lock().unwrap()[&self.id]
            .downcast_ref::<Mutex<S>>()
            .unwrap()
            .lock()
            .unwrap()
            .clone()
            .sub(rhs)
    }
}

impl<S: Div + Clone + 'static> Div<S> for State<S> {
    type Output = <S as Div>::Output;

    fn div(self, rhs: S) -> Self::Output {
        self.manager.store.lock().unwrap()[&self.id]
            .downcast_ref::<Mutex<S>>()
            .unwrap()
            .lock()
            .unwrap()
            .clone()
            .div(rhs)
    }
}

impl<S: AddAssign + 'static> AddAssign<S> for State<S> {
    fn add_assign(&mut self, rhs: S) {
        self.manager.store.lock().unwrap()[&self.id]
            .downcast_ref::<Mutex<S>>()
            .unwrap()
            .lock()
            .unwrap()
            .add_assign(rhs);
        self.dirty();
    }
}

impl<S: SubAssign + 'static> SubAssign<S> for State<S> {
    fn sub_assign(&mut self, rhs: S) {
        self.manager.store.lock().unwrap()[&self.id]
            .downcast_ref::<Mutex<S>>()
            .unwrap()
            .lock()
            .unwrap()
            .sub_assign(rhs);
        self.dirty();
    }
}

impl<S: PartialEq<S> + 'static> PartialEq<S> for State<S> {
    fn eq(&self, other: &S) -> bool {
        self.manager.store.lock().unwrap()[&self.id]
            .downcast_ref::<Mutex<S>>()
            .unwrap()
            .lock()
            .unwrap()
            .eq(other)
    }
}
