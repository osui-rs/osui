use std::sync::{Arc, Mutex};

use crate::{
    extensions::Handler,
    style::RawTransform,
    widget::{Element, Widget},
    RenderWrapperEvent,
};

pub struct VGrid {
    transform: Mutex<RawTransform>,
    color: u32,
    gap: u16,
}
pub struct HGrid {
    transform: Mutex<RawTransform>,
    color: u32,
    gap: u16,
}

impl Element for Arc<VGrid> {
    fn render(&mut self, scope: &mut crate::render_scope::RenderScope) {
        let transform = self.transform.lock().unwrap();
        let (w, h) = scope.get_size_or(transform.width, transform.height);
        scope.draw_rect(w, h, self.color);
    }

    fn after_render(&mut self, scope: &crate::render_scope::RenderScope) {
        let mut t = self.transform.lock().unwrap();
        let st = scope.get_transform();

        t.x = st.x;
        t.y = st.y;

        t.width = 0;
        t.height = 0;
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl VGrid {
    pub fn new(color: u32, gap: u16) -> Arc<Self> {
        Arc::new(Self {
            transform: Mutex::new(RawTransform::new()),
            color,
            gap,
        })
    }

    pub fn draw(self: &Arc<Self>, element: &Arc<Widget>) -> Arc<Widget> {
        let r2 = self.clone();
        element.component(Handler::new(move |elem, e: &RenderWrapperEvent| {
            let scope = e.get_scope();
            scope.clear();

            let mut transform = r2.transform.lock().unwrap();

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

            transform.height += if transform.height == 0 {
                elem_transform.height
            } else {
                elem_transform.height + r2.gap
            };
            transform.width = transform.width.max(elem_transform.width);

            elem_transform.x = transform.x;
            elem_transform.y = transform.y;
            transform.y += elem_transform.height + r2.gap;

            scope.draw();
            elem.0.lock().unwrap().after_render(&scope);

            scope.set_parent_size(w, h);
        }));
        element.clone()
    }
}

impl Element for Arc<HGrid> {
    fn render(&mut self, scope: &mut crate::render_scope::RenderScope) {
        let transform = self.transform.lock().unwrap();
        let (w, h) = scope.get_size_or(transform.width, transform.height);
        scope.draw_rect(w, h, self.color);
    }

    fn after_render(&mut self, scope: &crate::render_scope::RenderScope) {
        let mut t = self.transform.lock().unwrap();
        let st = scope.get_transform();

        t.x = st.x;
        t.y = st.y;

        t.width = 0;
        t.height = 0;
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl HGrid {
    pub fn new(color: u32, gap: u16) -> Arc<Self> {
        Arc::new(Self {
            transform: Mutex::new(RawTransform::new()),
            color,
            gap,
        })
    }

    pub fn draw(self: &Arc<Self>, element: &Arc<Widget>) -> Arc<Self> {
        let r2 = self.clone();
        let r = self.clone();
        element.component(Handler::new(move |elem, e: &RenderWrapperEvent| {
            let scope = e.get_scope();
            scope.clear();

            let mut transform = r2.transform.lock().unwrap();

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

            transform.width += if transform.width == 0 {
                elem_transform.width
            } else {
                elem_transform.width + r2.gap
            };
            transform.height = transform.height.max(elem_transform.height);

            elem_transform.x = transform.x;
            elem_transform.y = transform.y;
            transform.x += elem_transform.width + r2.gap;

            scope.draw();
            elem.0.lock().unwrap().after_render(&scope);

            scope.set_parent_size(w, h);
        }));
        r
    }
}
