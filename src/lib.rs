use std::any::Any;

use crate::{
    events::EventManager,
    extensions::{Extension, ExtensionManager},
    render_scope::RenderScope,
};

pub mod events;
pub mod extensions;
pub mod macros;
pub mod render_scope;
pub mod text;
pub mod utils;

pub trait Element {
    #[allow(unused)]
    fn render(&mut self, scope: &mut RenderScope) {}
    #[allow(unused)]
    fn init(&mut self, events: &mut EventManager) {}
    fn as_any(self: Box<Self>) -> Box<dyn Any>;
}

pub struct Screen {
    pub elements: Vec<Box<dyn Element>>,
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            elements: Vec::new(),
        }
    }

    pub fn draw<E: Element + 'static>(&mut self, element: E) {
        self.elements.push(Box::new(element));
    }

    pub fn run(
        &mut self,
        events: &mut EventManager,
        extensions: &mut ExtensionManager,
    ) -> std::io::Result<()> {
        let mut scope = RenderScope {};

        for elem in &mut self.elements {
            elem.init(events);
        }

        extensions.init(self, events);

        loop {
            extensions.tick_start(self, events);

            for elem in &mut self.elements {
                elem.render(&mut scope);
            }

            extensions.tick_end(self, events);
        }
    }
}
