use std::sync::{Arc, Mutex};

use crate::Node;

pub type Component = Arc<dyn Fn(&Arc<Context>) -> Vec<Node>>;

pub struct Context {
    component: Mutex<Component>,
    nodes: Mutex<Vec<Node>>,
}

impl Context {
    pub fn new<F: Fn(&Arc<Context>) -> Vec<Node> + 'static>(component: F) -> Arc<Self> {
        Arc::new(Self {
            component: Mutex::new(Arc::new(component)),
            nodes: Mutex::new(Vec::new()),
        })
    }

    pub fn refresh(self: &Arc<Context>) {
        let c = self.component.lock().unwrap().clone();
        *self.nodes.lock().unwrap() = (c)(self);
    }

    pub fn get_nodes(self: &Arc<Context>) -> Vec<Node> {
        self.nodes.lock().unwrap().clone()
    }
}
