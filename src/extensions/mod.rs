pub mod keypress;
pub mod tick_rate;

use std::sync::Arc;

use crate::{events::EventManager, Screen};

pub trait Extension {
    fn init(&mut self, screen: &mut Screen, events: &Arc<EventManager>);
    fn tick_start(&mut self, screen: &mut Screen, events: &Arc<EventManager>);
    fn tick_end(&mut self, screen: &mut Screen, events: &Arc<EventManager>);
}

pub struct ExtensionManager(Vec<Box<dyn Extension>>);

impl Extension for ExtensionManager {
    fn init(&mut self, screen: &mut Screen, events: &Arc<EventManager>) {
        for ext in &mut self.0 {
            ext.init(screen, events);
        }
    }

    fn tick_start(&mut self, screen: &mut Screen, events: &Arc<EventManager>) {
        for ext in &mut self.0 {
            ext.tick_start(screen, events);
        }
    }

    fn tick_end(&mut self, screen: &mut Screen, events: &Arc<EventManager>) {
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
