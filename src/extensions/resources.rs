use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct VecResource<T>(Arc<Mutex<Box<dyn FnMut(&T) + Send + Sync>>>);

impl<T: Send + Sync + 'static> VecResource<T> {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(Box::new(|_| {}))))
    }

    pub fn iterate<F: FnMut(&T) + Send + Sync + 'static>(&self, f: F) {
        *self.0.lock().unwrap() = Box::new(f);
    }

    pub fn push(&self, e: T) {
        let i = self.0.clone();

        std::thread::spawn(move || {
            (i.lock().unwrap())(&e);
        });
    }
}
