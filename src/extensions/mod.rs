pub mod id;
pub mod input_handling;
pub mod tick;
pub mod velocity;

pub use id::*;
pub use input_handling::*;
pub use tick::*;
pub use velocity::*;

use std::{
    any::{type_name, Any},
    fmt::Debug,
    sync::{Arc, Mutex},
};

use crate::{
    render_scope::RenderScope,
    widget::{Component, Widget},
    Screen,
};

pub trait Extension {
    fn init(&mut self, _screen: Arc<Screen>) {}
    fn on_close(&mut self, _screen: Arc<Screen>) {}
    fn render_widget(&mut self, _scope: &mut RenderScope, _widget: &Arc<Widget>) {}
}

pub trait Event: Send + Sync {
    fn as_any(&self) -> &dyn Any;
}

#[derive(Clone)]
pub struct Handler<E: Event>(Arc<Mutex<dyn FnMut(&Arc<Widget>, &E) + Send + Sync>>);

impl<E: Event + 'static> Component for Handler<E> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl<E: Event + 'static> Handler<E> {
    pub fn new<F: FnMut(&Arc<Widget>, &E) + Send + Sync + 'static>(f: F) -> Handler<E> {
        Handler(Arc::new(Mutex::new(f)))
    }

    pub fn call(&self, w: &Arc<Widget>, e: &E) {
        (self.0.lock().unwrap())(w, e)
    }
}

impl<E: Event> Debug for Handler<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Handler({})", type_name::<E>())
    }
}
