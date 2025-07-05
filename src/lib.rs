use std::sync::{Arc, Mutex};

use crate::{
    extensions::{Extension, Handler},
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
    pub widgets: Vec<Arc<Widget>>,
    extensions: Vec<Arc<Mutex<Box<dyn Extension>>>>,
}

event!(RenderWrapperEvent(*mut RenderScope));

impl RenderWrapperEvent {
    pub fn get_scope(&self) -> &mut RenderScope {
        unsafe { &mut *self.0 }
    }
}

unsafe impl Send for RenderWrapperEvent {}
unsafe impl Sync for RenderWrapperEvent {}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            widgets: Vec::new(),
            extensions: Vec::new(),
        }
    }

    pub fn draw<E: Element + 'static>(&mut self, element: E) -> &Arc<Widget> {
        self.widgets.push(Arc::new(Widget::new(Box::new(element))));
        self.widgets.last().unwrap()
    }

    pub fn extension<E: Extension + 'static>(&mut self, ext: E) {
        self.extensions.push(Arc::new(Mutex::new(Box::new(ext))));
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        for elem in &mut self.widgets {
            elem.component(Transform::new());
        }

        for ext in &self.extensions {
            ext.lock().unwrap().init(&self.widgets);
        }

        utils::hide_cursor()?;

        loop {
            self.render()?;
            std::thread::sleep(std::time::Duration::from_millis(28));
        }
    }

    pub fn render(&self) -> std::io::Result<()> {
        let mut scope = RenderScope::new();
        let (w, h) = crossterm::terminal::size().unwrap();
        scope.set_parent_size(w, h);

        utils::clear()?;
        for elem in &self.widgets {
            if let Some(wrapper) = elem.get::<Handler<RenderWrapperEvent>>() {
                wrapper.call(elem, &RenderWrapperEvent(&mut scope));
            } else {
                scope.clear();
                if let Some(t) = elem.get() {
                    scope.set_transform(&t);
                }

                for ext in &self.extensions {
                    ext.lock().unwrap().render_widget(&mut scope, elem);
                }

                elem.0.lock().unwrap().render(&mut scope);

                if let Some(t) = elem.get() {
                    scope.set_transform(&t);
                }
                scope.draw();

                elem.0.lock().unwrap().after_render(&scope);
            }
        }
        Ok(())
    }
}
