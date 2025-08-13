use std::sync::Arc;

use crate::{
    prelude::ElementRenderer,
    style::RawTransform,
    widget::{Element, Widget},
    NoRenderRoot,
};

pub struct RowRenderer<'a>(&'a mut RawTransform, u16, &'a mut u16);
pub struct ColumnRenderer<'a>(&'a mut RawTransform, u16, &'a mut u16);

pub struct FlexRow {
    pub gap: u16,
    children: Vec<Arc<Widget>>,
    size: (u16, u16),
}

pub struct FlexCol {
    pub gap: u16,
    children: Vec<Arc<Widget>>,
    size: (u16, u16),
}

impl FlexRow {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            size: (0, 0),
            gap: 0,
        }
    }
}

impl FlexCol {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            size: (0, 0),
            gap: 0,
        }
    }
}

impl Element for FlexRow {
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
        let transform_before = transform.clone();
        let (w, h) = scope.get_parent_size();
        scope.set_parent_size(transform.width, transform.height);
        let mut v = 0;

        let mut renderer = RowRenderer(&mut transform, self.gap, &mut v);

        for widget in &self.children {
            scope.render_widget(&mut renderer, render_context.get_context(), widget);
        }
        scope.set_parent_size(w, h);
        scope.set_transform_raw(transform_before);
        self.size = (transform.width, transform.height);
    }

    fn draw_child(&mut self, element: &Arc<Widget>) {
        self.children.push(element.clone());
        element.inject(|w| w.component(NoRenderRoot));
    }

    fn undraw_child(&mut self, element: &Arc<Widget>) {
        if let Some(i) = self.children.iter().position(|v| Arc::ptr_eq(v, element)) {
            self.children.remove(i);
        }
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

impl Element for FlexCol {
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
        let transform_before = transform.clone();
        let (w, h) = scope.get_parent_size();
        scope.set_parent_size(transform.width, transform.height);
        let mut v = 0;

        let mut renderer = ColumnRenderer(&mut transform, self.gap, &mut v);

        for widget in &self.children {
            scope.render_widget(&mut renderer, render_context.get_context(), widget);
        }
        scope.set_parent_size(w, h);
        scope.set_transform_raw(transform_before);
        self.size = (transform.width, transform.height);
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

impl ElementRenderer for RowRenderer<'_> {
    fn before_draw(&mut self, scope: &mut crate::prelude::RenderScope, _widget: &Arc<Widget>) {
        let t = scope.get_transform_mut();
        self.0.width = self.0.width.max(*self.2 + t.width + (t.px * 2));
        self.0.height = self.0.height.max(t.height + (t.py * 2));

        t.x += self.0.x;
        t.y += self.0.y + *self.2;
        *self.2 += t.height + self.1 + (t.py * 2);

        t.px += self.0.px;
        t.py += self.0.py;
    }
}

impl ElementRenderer for ColumnRenderer<'_> {
    fn before_draw(&mut self, scope: &mut crate::prelude::RenderScope, _widget: &Arc<Widget>) {
        let t = scope.get_transform_mut();
        self.0.width = self.0.width.max(*self.2 + t.width + (t.px * 2));
        self.0.height = self.0.height.max(t.height + (t.py * 2));

        t.x += self.0.x + *self.2;
        t.y += self.0.y;
        *self.2 += t.width + self.1 + (t.px * 2);

        t.px += self.0.px;
        t.py += self.0.py;
    }
}
