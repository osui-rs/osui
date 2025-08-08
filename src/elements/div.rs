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
        _: &crate::render_scope::RenderContext,
    ) {
        let (width, height) = scope.get_size_or(self.size.0, self.size.1);
        scope.use_area(width, height);
    }

    fn after_render(
        &mut self,
        scope: &mut crate::render_scope::RenderScope,
        render_context: &crate::render_scope::RenderContext,
    ) {
        let mut transform = scope.get_transform().clone();
        let mut renderer = DivRenderer(&mut transform);

        let (w, h) = scope.get_parent_size();
        scope.set_parent_size(renderer.0.width, renderer.0.height);

        for widget in &self.children {
            scope.render_widget(&mut renderer, render_context.get_context(), widget);
        }
        scope.set_parent_size(w, h);

        self.size = (renderer.0.width, renderer.0.height);
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

impl ElementRenderer for DivRenderer<'_> {
    fn before_draw(&mut self, scope: &mut crate::prelude::RenderScope, _widget: &Arc<Widget>) {
        let t = scope.get_transform_mut();
        self.0.width = self.0.width.max(t.width + (t.px * 2));
        self.0.height = self.0.height.max(t.height + (t.py * 2));
        t.x += self.0.x + self.0.px;
        t.y += self.0.y + self.0.py;
    }
}
