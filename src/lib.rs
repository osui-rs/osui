use std::sync::Arc;

use crate::{
    render_scope::RenderScope,
    style::Transform,
    widget::{Element, Widget},
};

pub mod elements;
pub mod macros;
pub mod render_scope;
pub mod style;
pub mod utils;
pub mod widget;

pub struct Screen {
    pub elements: Vec<Arc<Widget>>,
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            elements: Vec::new(),
        }
    }

    pub fn draw<E: Element + 'static>(&mut self, element: E) -> &Arc<Widget> {
        self.elements.push(Arc::new(Widget::new(Box::new(element))));
        self.elements.last().unwrap()
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        for elem in &mut self.elements {
            elem.component(Transform::new());
        }

        utils::hide_cursor()?;

        self.render()?;

        loop {}
    }

    pub fn render(&mut self) -> std::io::Result<()> {
        let mut scope = RenderScope::new();

        utils::clear()?;
        for elem in &mut self.elements {
            scope.clear();
            if let Some(t) = elem.get() {
                scope.set_transform(&t);
            }

            elem.0.lock().unwrap().render(&mut scope);

            if let Some(t) = elem.get() {
                scope.set_transform(&t);
            }
            scope.draw();
        }
        Ok(())
    }
}
