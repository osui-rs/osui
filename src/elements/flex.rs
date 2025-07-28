use std::sync::{Arc, Mutex};

use crate::{
    extensions::Handler,
    style::RawTransform,
    widget::{Element, Widget},
    RenderWrapperEvent,
};

pub struct FlexRow {
    transform: Mutex<RawTransform>,
    color: u32,
    gap: u16,
}
pub struct FlexCol {
    transform: Mutex<RawTransform>,
    color: u32,
    gap: u16,
}

impl Element for Arc<FlexRow> {
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

    fn draw_child(&self, element: &Arc<Widget>) {
        let r = self.clone();
        element.inject(move |w| {
            let r = r.clone();
            w.component(Handler::new(move |elem, e: &RenderWrapperEvent| {
                let scope = e.get_scope();
                scope.clear();

                let mut transform = r.transform.lock().unwrap();

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
                    elem_transform.height + r.gap
                };
                transform.width = transform.width.max(elem_transform.width);

                elem_transform.x = transform.x;
                elem_transform.y = transform.y;
                transform.y += elem_transform.height + r.gap;

                scope.draw();
                elem.0.lock().unwrap().after_render(&scope);

                scope.set_parent_size(w, h);
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

impl FlexRow {
    pub fn new(color: u32, gap: u16) -> Arc<Self> {
        Arc::new(Self {
            transform: Mutex::new(RawTransform::new()),
            color,
            gap,
        })
    }
}

impl Element for Arc<FlexCol> {
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

    fn draw_child(&self, element: &Arc<Widget>) {
        let r = self.clone();
        element.inject(move |w| {
            let r = r.clone();
            w.component(Handler::new(move |elem, e: &RenderWrapperEvent| {
                let scope = e.get_scope();
                scope.clear();

                let mut transform = r.transform.lock().unwrap();

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
                    elem_transform.width + r.gap
                };
                transform.height = transform.height.max(elem_transform.height);

                elem_transform.x = transform.x;
                elem_transform.y = transform.y;
                transform.x += elem_transform.width + r.gap;

                scope.draw();
                elem.0.lock().unwrap().after_render(&scope);

                scope.set_parent_size(w, h);
            }))
        });
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl FlexCol {
    pub fn new(color: u32, gap: u16) -> Arc<Self> {
        Arc::new(Self {
            transform: Mutex::new(RawTransform::new()),
            color,
            gap,
        })
    }
}
