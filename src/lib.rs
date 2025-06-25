use std::any::Any;

use crate::{
    event_manager::{Event, EventManager},
    render_scope::RenderScope,
};

pub mod event_manager;
pub mod extensions;
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

pub trait Extension {
    fn init(&mut self, screen: &mut Screen, events: &mut EventManager);
    fn tick_start(&mut self, screen: &mut Screen, events: &mut EventManager);
    fn tick_end(&mut self, screen: &mut Screen, events: &mut EventManager);
}

pub struct ExtensionManager(Vec<Box<dyn Extension>>);

impl Extension for ExtensionManager {
    fn init(&mut self, screen: &mut Screen, events: &mut EventManager) {
        for ext in &mut self.0 {
            ext.init(screen, events);
        }
    }

    fn tick_start(&mut self, screen: &mut Screen, events: &mut EventManager) {
        for ext in &mut self.0 {
            ext.tick_start(screen, events);
        }
    }

    fn tick_end(&mut self, screen: &mut Screen, events: &mut EventManager) {
        for ext in &mut self.0 {
            ext.tick_end(screen, events);
        }
    }
}

impl ExtensionManager {
    pub fn new() -> ExtensionManager {
        ExtensionManager(Vec::new())
    }
    pub fn add<E: Extension + 'static>(&mut self, extension: E) {
        self.0.push(Box::new(extension));
    }
}

pub struct EventHandlerFn(pub Box<dyn FnMut(Box<dyn Event>)>);

impl std::fmt::Debug for EventHandlerFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FnMut()")
    }
}

#[derive(Debug, Clone)]
pub struct Close;

impl Event for Close {
    fn as_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
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
        extension_manager: &mut ExtensionManager,
    ) -> std::io::Result<()> {
        let mut scope = RenderScope {};

        for elem in &mut self.elements {
            elem.init(events);
        }

        extension_manager.init(self, events);

        loop {
            extension_manager.tick_start(self, events);

            for elem in &mut self.elements {
                elem.render(&mut scope);
            }

            extension_manager.tick_end(self, events);
        }
    }
}
