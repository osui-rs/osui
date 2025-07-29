use std::sync::{Arc, Mutex};

use crate::widget::{Element, Widget};

pub struct Div {
    color: u32,
    children: Mutex<Vec<Arc<Widget>>>,
}

impl Element for Arc<Div> {
    fn render(&mut self, scope: &mut crate::render_scope::RenderScope) {
        let (width, height) = scope.get_size_or_parent();
        scope.draw_rect(width, height, self.color);
    }

    fn after_render(&mut self, scope: &mut crate::render_scope::RenderScope) {
        let transform = scope.get_transform().clone();

        scope.draw();
        let (w, h) = scope.get_parent_size();
        scope.set_parent_size(transform.width, transform.height);

        for elem in self.children.lock().unwrap().iter() {
            scope.clear();
            if let Some(t) = elem.get() {
                scope.set_transform(&t);
            }
            elem.0.lock().unwrap().render(scope);
            if let Some(t) = elem.get() {
                scope.set_transform(&t);
            }
            let t = scope.get_transform_mut();
            t.x += transform.x;
            t.y += transform.y;

            scope.draw();
            elem.0.lock().unwrap().after_render(scope);
            scope.clear();
        }
        scope.set_parent_size(w, h);
    }

    fn draw_child(&self, element: &Arc<Widget>) {
        self.children.lock().unwrap().push(element.clone());
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
            children: Mutex::new(Vec::new()),
        })
    }
}
