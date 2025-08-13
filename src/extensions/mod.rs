pub mod focus;
pub mod id;
pub mod input_handling;
pub mod tick;
pub mod velocity;

pub use focus::*;
pub use id::*;
pub use input_handling::*;
pub use tick::*;
pub use velocity::*;

use std::{
    any::{type_name, Any},
    fmt::Debug,
    sync::{Arc, Mutex, MutexGuard},
};

use crate::{
    render_scope::RenderScope,
    widget::{Component, Widget},
    Screen,
};

pub trait Extension {
    fn init(&mut self, _ctx: &Context) {}
    fn event(&mut self, _ctx: &Context, _event: &dyn Event) {}
    fn on_close(&mut self) {}
    fn render(&mut self, _ctx: &Context, _scope: &mut RenderScope) {}
    fn render_widget(&mut self, _ctx: &Context, _scope: &mut RenderScope, _widget: &Arc<Widget>) {}
    fn after_render_widget(
        &mut self,
        _ctx: &Context,
        _scope: &mut RenderScope,
        _widget: &Arc<Widget>,
    ) {
    }
}

pub trait Event: Send + Sync {
    fn as_any(&self) -> &dyn Any;
}

#[derive(Clone)]
pub struct Context {
    screen: Arc<Screen>,
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

impl<'a> dyn Event + 'a {
    pub fn get<T: Event + 'static>(&self) -> Option<&T> {
        self.as_any().downcast_ref()
    }
}

impl Context {
    pub fn new(screen: Arc<Screen>) -> Self {
        Self { screen }
    }

    pub fn event<E: Event + Clone + 'static>(&self, e: &E) {
        for widget in self.screen.widgets.lock().unwrap().iter() {
            widget.event(e);
        }

        for ext in self.screen.extensions.lock().unwrap().iter() {
            ext.lock().unwrap().event(self, e);
        }
    }

    pub fn get_widgets(&'_ self) -> MutexGuard<'_, Vec<Arc<Widget>>> {
        self.screen.widgets.lock().unwrap()
    }

    pub fn iter_components<C: Component + 'static + Clone, F: FnMut(&Arc<Widget>, Option<C>)>(
        &self,
        mut iterator: F,
    ) {
        for widget in self.screen.widgets.lock().unwrap().iter() {
            iterator(widget, widget.get());
        }
    }

    pub fn get_components<C: Component + 'static + Clone>(&self) -> Vec<C> {
        let mut components = Vec::new();
        for widget in self.screen.widgets.lock().unwrap().iter() {
            if let Some(c) = widget.get() {
                components.push(c);
            }
        }
        components
    }

    pub fn render_root(&self, scope: &mut RenderScope) {
        for ext in self.screen.extensions.lock().unwrap().iter() {
            ext.lock().unwrap().render(self, scope);
        }
    }

    pub fn render(&self, w: &Arc<Widget>, scope: &mut RenderScope) {
        for ext in self.screen.extensions.lock().unwrap().iter() {
            ext.lock().unwrap().render_widget(self, scope, w);
        }
    }

    pub fn after_render(&self, w: &Arc<Widget>, scope: &mut RenderScope) {
        for ext in self.screen.extensions.lock().unwrap().iter() {
            ext.lock().unwrap().after_render_widget(self, scope, w);
        }
    }
}
