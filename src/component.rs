use std::sync::Arc;

use crate::Node;

pub type Component = Arc<dyn Fn(&mut Context) -> Vec<Node>>;

pub struct Context {
    component: Component,
    nodes: Vec<Node>,
}

impl Context {
    pub fn new<F: Fn(&mut Context) -> Vec<Node> + 'static>(component: F) -> Self {
        Self {
            component: Arc::new(component),
            nodes: Vec::new(),
        }
    }

    pub fn refresh(&mut self) {
        self.nodes = (self.component.clone())(self);
    }

    pub fn get_nodes(&self) -> Vec<Node> {
        self.nodes.clone()
    }
}
