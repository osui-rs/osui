pub mod id;
pub mod tick;
pub mod velocity;

use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use crate::{render_scope::RenderScope, widget::Widget};

pub trait Extension {
    fn init(&mut self, _widgets: &Vec<Arc<Widget>>) {}
    fn render_widget(&mut self, _scope: &mut RenderScope, _widget: &Arc<Widget>) {}
}

#[derive(Clone)]
pub struct Handler(pub Arc<Mutex<dyn FnMut(&Arc<Widget>) + Send + Sync>>);

impl Handler {
    pub fn new<F: FnMut(&Arc<Widget>) + Send + Sync + 'static>(f: F) -> Handler {
        Handler(Arc::new(Mutex::new(f)))
    }

    pub fn call(&self, w: &Arc<Widget>) {
        (self.0.lock().unwrap())(w)
    }
}

impl Debug for Handler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Handler")
    }
}
