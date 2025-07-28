use std::sync::{Arc, Mutex};

use crate::{
    extensions::Handler,
    widget::{Element, Widget},
    RenderWrapperEvent,
};

pub struct Div {
    color: u32,
    children: Mutex<usize>,
    rendered: Mutex<usize>,
}

impl Element for Arc<Div> {
    fn render(&mut self, scope: &mut crate::render_scope::RenderScope) {
        let (w, h) = scope.get_size_or_parent();
        scope.draw_rect(w, h, self.color);
    }

    fn draw_child(&self, element: &Arc<Widget>) {
        *self.children.lock().unwrap() += 1;
        let r = self.clone();
        element.inject(move |w| {
            let r = r.clone();
            w.component(Handler::new(move |elem, e: &RenderWrapperEvent| {
                let scope = e.get_scope();
                let transform = scope.get_transform().clone();

                scope.clear();
                let (w, h) = scope.get_parent_size();
                scope.set_parent_size(transform.width, transform.height);

                if let Some(t) = elem.get() {
                    scope.set_transform(&t);
                }

                elem.0.lock().unwrap().render(scope);

                if let Some(t) = elem.get() {
                    scope.set_transform(&t);
                }

                let elem_transform = scope.get_transform_mut();
                elem_transform.x += transform.x;
                elem_transform.y += transform.y;

                scope.draw();
                elem.0.lock().unwrap().after_render(&scope);

                scope.set_parent_size(w, h);

                {
                    let mut rendered = r.rendered.lock().unwrap();
                    let children = r.children.lock().unwrap();
                    *rendered += 1;
                    if *rendered < *children {
                        scope.set_transform_raw(transform);
                    } else {
                        *rendered = 0;
                    }
                }
            }))
        })
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Div {
    pub fn new(color: u32) -> Arc<Self> {
        Arc::new(Div {
            color,
            children: Mutex::new(0),
            rendered: Mutex::new(0),
        })
    }
}
