use std::any::Any;

use crate::renderer::{OsuiRenderer, Renderer};

pub mod error;
pub mod renderer;
pub mod state;

pub use error::Result;

pub type Node = Box<dyn Fn(&mut dyn Renderer)>;
pub type Component = Box<dyn Fn(&mut Context) -> Node>;

pub trait Event {
    fn as_any(&self) -> &dyn Any;
}

pub struct Context {
    event_handlers: Vec<Box<dyn Fn(&dyn Event)>>,
}

pub struct Osui {
    node: Node,
    renderer: Option<Box<dyn Renderer>>,
}

impl Osui {
    pub fn new(component: Component) -> Self {
        let mut cx = Context {
            event_handlers: Vec::new(),
        };
        Osui {
            node: component(&mut cx),
            renderer: None,
        }
    }
    pub fn with_renderer(component: Component, renderer: Box<dyn Renderer>) -> Self {
        let mut cx = Context {
            event_handlers: Vec::new(),
        };
        Osui {
            node: component(&mut cx),
            renderer: Some(renderer),
        }
    }

    pub fn render(&mut self) -> Result<()> {
        renderer::utils::clear()?;
        if let Some(renderer) = &mut self.renderer {
            (self.node)(renderer.as_mut());
        } else {
            (self.node)(&mut OsuiRenderer);
        }
        Ok(())
    }
}

impl Context {
    pub fn event<E: Event + 'static, F: Fn(&E) + 'static>(&mut self, handler: F) {
        self.event_handlers.push(Box::new(move |e| {
            if let Some(e) = e.as_any().downcast_ref() {
                (handler)(e)
            }
        }));
    }

    pub fn global_event<E: Event + 'static, F: Fn(&E) + 'static>(&mut self, handler: F) {
        self.event_handlers.push(Box::new(move |e| {
            if let Some(e) = e.as_any().downcast_ref() {
                (handler)(e)
            }
        }));
    }
}
