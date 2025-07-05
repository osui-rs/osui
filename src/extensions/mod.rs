pub mod id;
pub mod tick;
pub mod velocity;

use std::{
    any::{Any, TypeId},
    fmt::Debug,
    sync::{Arc, Mutex},
};

use crate::{
    render_scope::RenderScope,
    widget::{Component, Widget},
};

pub trait Extension {
    fn init(&mut self, _widgets: &Vec<Arc<Widget>>) {}
    fn render_widget(&mut self, _scope: &mut RenderScope, _widget: &Arc<Widget>) {}
}

pub trait Event: Send + Sync {
    fn as_any(&self) -> &dyn Any;
}

#[derive(Clone)]
pub struct Handler(
    Arc<Mutex<dyn FnMut(&Arc<Widget>, &dyn Event) + Send + Sync>>,
    TypeId,
);

impl Component for Handler {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Handler {
    pub fn new<E: Event + 'static, F: FnMut(&Arc<Widget>, &E) + Send + Sync + 'static>(
        mut f: F,
    ) -> Handler {
        Handler(
            Arc::new(Mutex::new(move |w: &Arc<Widget>, e: &dyn Event| {
                if let Some(e) = e.as_any().downcast_ref::<E>() {
                    f(w, e)
                }
            })),
            TypeId::of::<E>(),
        )
    }

    pub fn call<E: Event>(&self, w: &Arc<Widget>, e: &E) {
        if e.as_any().type_id() == self.1 {
            (self.0.lock().unwrap())(w, e)
        }
    }
}

impl Debug for Handler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Handler")
    }
}
