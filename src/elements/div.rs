use std::sync::Arc;

use crate::{
    prelude::ElementRenderer,
    style::RawTransform,
    widget::{Element, Widget},
    NoRenderRoot,
};

pub struct DivRenderer<'a>(pub &'a mut RawTransform);

pub struct Div {
    children: Vec<Arc<Widget>>,
    render: (u16, u16),
}

impl Div {
    pub fn new() -> Self {
        Div {
            children: Vec::new(),
            render: (0, 0),
        }
    }
}

impl Element for Div {
    fn render(
        &mut self,
        scope: &mut crate::prelude::RenderScope,
        _: &crate::render_scope::RenderContext,
    ) {
        scope.use_area(self.render.0, self.render.1);
    }

    fn after_render(
        &mut self,
        parent_scope: &mut crate::render_scope::RenderScope,
        render_context: &crate::render_scope::RenderContext,
    ) {
        let mut transform = {
            let (width, height) = parent_scope.get_size_or_parent();
            let mut t = parent_scope.get_transform().clone();
            t.width = width;
            t.height = height;
            t.transform_parent(parent_scope.get_parent_transform());
            t
        };

        let mut scope = crate::render_scope::RenderScope::new();
        scope.set_parent_transform(transform.clone());

        transform.height = 0;
        let mut renderer = DivRenderer(&mut transform);
        for widget in &self.children {
            scope.render_widget(&mut renderer, render_context.get_context(), widget);
        }

        self.render = (transform.width, transform.height);
    }

    fn draw_child(&mut self, element: &Arc<Widget>) {
        self.children.push(element.clone());
        element.inject(|w| w.component(NoRenderRoot));
    }

    fn is_ghost(&mut self) -> bool {
        true
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl ElementRenderer for DivRenderer<'_> {
    fn before_draw(&mut self, scope: &mut crate::prelude::RenderScope, _widget: &Arc<Widget>) {
        let t = scope.get_transform_mut();
        self.0.width = self.0.width.max(t.width + (t.px * 2));
        self.0.height = self.0.height.max(t.height + (t.py * 2));
        t.x += self.0.x + self.0.px;
        t.y += self.0.y + self.0.py;
    }
}
