use std::sync::{Arc, Mutex};

use crate::{component, extensions::Extension, widget::Widget};

pub struct IdExtension(Mutex<Vec<Arc<Widget>>>);

impl Extension for Arc<IdExtension> {
    fn init(&mut self, widgets: &Vec<Arc<Widget>>) {
        self.0.lock().unwrap().append(&mut widgets.clone());
    }
}

impl IdExtension {
    pub fn new() -> Arc<Self> {
        Arc::new(Self(Mutex::new(Vec::new())))
    }

    pub fn get_element(self: &Arc<IdExtension>, id: usize) -> Option<Arc<Widget>> {
        for elem in self.0.lock().unwrap().iter() {
            if let Some(current_id) = elem.get::<Id>() {
                if current_id.0 == id {
                    return Some(elem.clone());
                }
            }
        }

        None
    }
}

component!(Id(pub usize));
