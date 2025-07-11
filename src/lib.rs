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
    pub widgets: Mutex<Vec<Arc<Widget>>>,
    extensions: Mutex<Vec<Arc<Mutex<Box<dyn Extension + Send + Sync>>>>>,
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
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            widgets: Mutex::new(Vec::new()),
            extensions: Mutex::new(Vec::new()),
        })
    }

    pub fn draw<E: Element + 'static>(self: &Arc<Self>, element: E) -> Arc<Widget> {
        let w = Arc::new(Widget::new(Box::new(element)));
        self.widgets.lock().unwrap().push(w.clone());
        w
    }

    pub fn extension<E: Extension + Send + Sync + 'static>(self: &Arc<Self>, ext: E) {
        self.extensions
            .lock()
            .unwrap()
            .push(Arc::new(Mutex::new(Box::new(ext))));
    }

    pub fn run(self: &Arc<Self>) -> std::io::Result<()> {
        for elem in self.widgets.lock().unwrap().iter() {
            elem.component(Transform::new());
        }

        for ext in self.extensions.lock().unwrap().iter() {
            ext.lock().unwrap().init(self.clone());
        }

        utils::hide_cursor()?;

        loop {
            self.render()?;
            std::thread::sleep(std::time::Duration::from_millis(28));
        }
    }

    pub fn render(self: &Arc<Self>) -> std::io::Result<()> {
        let mut scope = RenderScope::new();
        let (w, h) = crossterm::terminal::size().unwrap();
        scope.set_parent_size(w, h);

        utils::clear()?;
        for elem in self.widgets.lock().unwrap().iter() {
            if let Some(wrapper) = elem.get::<Handler<RenderWrapperEvent>>() {
                wrapper.call(elem, &RenderWrapperEvent(&mut scope));
            } else {
                scope.clear();
                if let Some(t) = elem.get() {
                    scope.set_transform(&t);
                }

                for ext in self.extensions.lock().unwrap().iter() {
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
