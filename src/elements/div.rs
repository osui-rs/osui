use std::sync::Arc;

use crate::widget::{Element, Widget};

#[derive(Default)]
pub struct Div {
    pub children: Vec<Arc<Widget>>,
}

impl Element for Div {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Div {
    pub fn new() -> Self {
        Div {
            children: Vec::new(),
        }
    }

    pub fn draw<E: Element + 'static>(&mut self, element: E) -> &Arc<Widget> {
        self.children.push(Arc::new(Widget::new(Box::new(element))));
        self.children.last().unwrap()
    }
}
