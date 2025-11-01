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
}

pub struct FlexCol {
    pub gap: u16,
    children: Vec<Arc<Widget>>,
}

impl FlexRow {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            gap: 0,
        }
    }
}

impl FlexCol {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            gap: 0,
        }
    }
}

impl Element for FlexRow {
    fn render(
        &mut self,
        parent_scope: &mut crate::render_scope::RenderScope,
        render_context: &crate::render_scope::RenderContext,
    ) {
        let mut transform = parent_scope.get_transform().clone();
        let (w, h) = parent_scope.get_size_or_parent();

        let mut scope = crate::render_scope::RenderScope::new();
        scope.set_parent_size(w, h);

        let mut v = 0;
        let mut renderer = RowRenderer(&mut transform, self.gap, &mut v);
        for widget in &self.children {
            scope.render_widget(&mut renderer, render_context.get_context(), widget);
        }

        parent_scope.use_area(w, h);
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
        parent_scope: &mut crate::render_scope::RenderScope,
        render_context: &crate::render_scope::RenderContext,
    ) {
        let mut transform = parent_scope.get_transform().clone();
        let (w, h) = parent_scope.get_size_or_parent();

        let mut scope = crate::render_scope::RenderScope::new();
        scope.set_parent_size(w, h);

        let mut v = 0;
        let mut renderer = ColumnRenderer(&mut transform, self.gap, &mut v);
        for widget in &self.children {
            scope.render_widget(&mut renderer, render_context.get_context(), widget);
        }

        parent_scope.use_area(w, h);
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
