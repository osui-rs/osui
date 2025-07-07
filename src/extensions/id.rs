use std::sync::Arc;

use crate::{component, extensions::Extension, widget::Widget, Screen};

pub struct IdExtension(pub Arc<Screen>);

impl Extension for Arc<IdExtension> {}

impl IdExtension {
    pub fn new(screen: Arc<Screen>) -> Arc<Self> {
        Arc::new(IdExtension(screen))
    }

    pub fn get_element(self: &Arc<IdExtension>, id: usize) -> Option<Arc<Widget>> {
        for elem in self.0.widgets.lock().unwrap().iter() {
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
