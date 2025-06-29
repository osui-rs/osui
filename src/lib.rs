use std::sync::Arc;

use crate::{
    extensions::Extension,
    render_scope::RenderScope,
    style::Transform,
    widget::{Element, Widget},
};

pub mod elements;
pub mod extensions;
pub mod macros;
pub mod render_scope;
pub mod style;
pub mod utils;
pub mod widget;

pub struct Screen {
    pub elements: Vec<Arc<Widget>>,
    extensions: Vec<Arc<Box<dyn Extension>>>,
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            elements: Vec::new(),
            extensions: Vec::new(),
        }
    }

    pub fn draw<E: Element + 'static>(&mut self, element: E) -> &Arc<Widget> {
        self.elements.push(Arc::new(Widget::new(Box::new(element))));
        self.elements.last().unwrap()
    }

    pub fn extension<E: Extension + 'static>(&mut self, ext: E) {
        self.extensions.push(Arc::new(Box::new(ext)));
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        for elem in &mut self.elements {
            elem.component(Transform::new());
        }

        utils::hide_cursor()?;

        loop {
            self.render()?;
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }

    pub fn render(&self) -> std::io::Result<()> {
        let mut scope = RenderScope::new();

        utils::clear()?;
        for elem in &self.elements {
            scope.clear();
            if let Some(t) = elem.get() {
                scope.set_transform(&t);
            }

            self.render_extension(elem.clone()).unwrap();
            elem.0.lock().unwrap().render(&mut scope);

            if let Some(t) = elem.get() {
                scope.set_transform(&t);
            }
            scope.draw();
        }
        Ok(())
    }

    pub fn render_extension(&self, wi: Arc<Widget>) -> std::io::Result<()> {
        for ext in &self.extensions {
            ext.render(&wi);
        }
        Ok(())
    }
}
