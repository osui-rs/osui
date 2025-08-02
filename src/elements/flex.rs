use std::sync::{Arc, Mutex};

use crate::{
    widget::{Element, Widget},
    NoRender,
};

pub struct FlexRow {
    gap: u16,
    children: Mutex<Vec<Arc<Widget>>>,
    size: (u16, u16),
}

pub struct FlexCol {
    gap: u16,
    children: Mutex<Vec<Arc<Widget>>>,
    size: (u16, u16),
}

impl FlexRow {
    pub fn new(gap: u16) -> Self {
        Self {
            children: Mutex::new(Vec::new()),
            size: (0, 0),
            gap,
        }
    }
}

impl FlexCol {
    pub fn new(gap: u16) -> Self {
        Self {
            children: Mutex::new(Vec::new()),
            size: (0, 0),
            gap,
        }
    }
}

impl Element for FlexRow {
    fn render(&mut self, scope: &mut crate::render_scope::RenderScope) {
        let (width, height) = scope.get_size_or(self.size.0, self.size.1);
        scope.use_area(width, height);
    }

    fn after_render(&mut self, scope: &mut crate::render_scope::RenderScope) {
        let mut transform = scope.get_transform().clone();
        let (w, h) = scope.get_parent_size();
        scope.set_parent_size(transform.width, transform.height);

        let mut v = 0;

        for elem in self.children.lock().unwrap().iter() {
            scope.clear();
            if let Some(style) = elem.get() {
                scope.set_style(style);
            }
            if let Some(t) = elem.get() {
                scope.set_transform(&t);
            }
            elem.get_elem().render(scope);
            if let Some(t) = elem.get() {
                scope.set_transform(&t);
            }
            let t = scope.get_transform_mut();
            transform.width = transform.width.max(t.width);
            transform.height = transform.height.max(v + t.height);
            t.x += transform.x;
            t.y += transform.y + v;
            v += t.height + self.gap + (t.py * 2);
            t.px += transform.px;
            t.py += transform.py;

            scope.draw();

            elem.get_elem().after_render(scope);
        }
        scope.set_parent_size(w, h);
        self.size = (transform.width, transform.height);
    }

    fn draw_child(&self, element: &Arc<Widget>) {
        self.children.lock().unwrap().push(element.clone());
        element.inject(|w| w.component(NoRender));
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Element for FlexCol {
    fn render(&mut self, scope: &mut crate::render_scope::RenderScope) {
        let (width, height) = scope.get_size_or(self.size.0, self.size.1);
        scope.use_area(width, height);
    }

    fn after_render(&mut self, scope: &mut crate::render_scope::RenderScope) {
        let mut transform = scope.get_transform().clone();
        let (w, h) = scope.get_parent_size();
        scope.set_parent_size(transform.width, transform.height);

        let mut v = 0;

        for elem in self.children.lock().unwrap().iter() {
            scope.clear();
            if let Some(style) = elem.get() {
                scope.set_style(style);
            }
            if let Some(t) = elem.get() {
                scope.set_transform(&t);
            }
            elem.get_elem().render(scope);
            if let Some(t) = elem.get() {
                scope.set_transform(&t);
            }
            let t = scope.get_transform_mut();
            transform.width = transform.width.max(v + t.width + (t.px * 2));
            transform.height = transform.height.max(t.height + (t.py * 2));
            t.x += transform.x + v;
            t.y += transform.y;
            v += t.width + self.gap + (t.px * 2);
            t.px += transform.px;
            t.py += transform.py;

            scope.draw();

            elem.get_elem().after_render(scope);
        }
        scope.set_parent_size(w, h);
        self.size = (transform.width, transform.height);
    }

    fn draw_child(&self, element: &Arc<Widget>) {
        self.children.lock().unwrap().push(element.clone());
        element.inject(|w| w.component(NoRender));
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
