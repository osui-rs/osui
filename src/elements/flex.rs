use std::sync::Arc;

use crate::{
    widget::{Element, Widget},
    NoRender, NoRenderRoot,
};

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

        let mut v = 0;

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
            transform.width = transform.width.max(t.width);
            transform.height = transform.height.max(v + t.height);
            t.x += transform.x;
            t.y += transform.y + v;
            v += t.height + self.gap + (t.py * 2);
            t.px += transform.px;
            t.py += transform.py;

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

impl Element for FlexCol {
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

        let mut v = 0;

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
            transform.width = transform.width.max(v + t.width + (t.px * 2));
            transform.height = transform.height.max(t.height + (t.py * 2));
            t.x += transform.x + v;
            t.y += transform.y;
            v += t.width + self.gap + (t.px * 2);
            t.px += transform.px;
            t.py += transform.py;

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
