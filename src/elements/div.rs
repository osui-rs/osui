use std::sync::Arc;

use crate::{
    widget::{Element, Widget},
    NoRender, NoRenderRoot,
};

pub struct Div {
    children: Vec<Arc<Widget>>,
    size: (u16, u16),
}

impl Div {
    pub fn new() -> Self {
        Div {
            children: Vec::new(),
            size: (0, 0),
        }
    }
}

impl Element for Div {
    fn render(
        &mut self,
        scope: &mut crate::render_scope::RenderScope,
        _: &crate::extensions::Context,
    ) {
        let (width, height) = scope.get_size_or(self.size.0, self.size.1);
        scope.use_area(width, height);
    }

    fn after_render(
        &mut self,
        scope: &mut crate::render_scope::RenderScope,
        ctx: &crate::extensions::Context,
    ) {
        let mut transform = scope.get_transform().clone();
        let (w, h) = scope.get_parent_size();
        scope.set_parent_size(transform.width, transform.height);

        for elem in &self.children {
            if elem.get::<NoRender>().is_some() {
                continue;
            }

            scope.clear();

            if let Some(style) = elem.get() {
                scope.set_style(style);
            }
            if let Some(t) = elem.get() {
                scope.set_transform(&t);
            }
            elem.get_elem().render(scope, ctx);
            ctx.render(elem, scope);
            if let Some(t) = elem.get() {
                scope.set_transform(&t);
            }
            let t = scope.get_transform_mut();
            transform.width = transform.width.max(t.x + t.width + (t.px * 2));
            transform.height = transform.height.max(t.y + t.height + (t.py * 2));
            t.x += transform.x + transform.px;
            t.y += transform.y + transform.py;

            scope.draw();
            elem.get_elem().after_render(scope, ctx);
            ctx.after_render(elem, scope);
        }
        scope.set_parent_size(w, h);
        self.size = (transform.width, transform.height);
    }

    fn draw_child(&mut self, element: &Arc<Widget>) {
        self.children.push(element.clone());
        element.inject(|w| w.component(NoRenderRoot));
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
