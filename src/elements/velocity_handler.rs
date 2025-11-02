use std::sync::Arc;

use crate::{
    component,
    prelude::ElementRenderer,
    style::RawTransform,
    widget::{Element, Widget},
    NoRenderRoot,
};

pub struct VelocityRenderer<'a>(&'a mut RawTransform, &'a mut Vec<(i32, i32)>, usize, u16);

pub struct VelocityHandler {
    children: Vec<Arc<Widget>>,
    progress: Vec<(i32, i32)>,
    render: (u16, u16),
    tick: u16,
}

impl VelocityHandler {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            progress: Vec::new(),
            render: (0, 0),
            tick: 0,
        }
    }
}

impl Element for VelocityHandler {
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

        transform.width = 0;
        transform.height = 0;
        let mut renderer = VelocityRenderer(&mut transform, &mut self.progress, 0, self.tick);
        for widget in &self.children {
            scope.render_widget(&mut renderer, render_context.get_context(), widget);
        }

        self.render = (transform.width, transform.height);

        if self.tick == 1000 {
            self.tick = 0;
        } else {
            self.tick += 1;
        }
    }

    fn draw_child(&mut self, element: &Arc<Widget>) {
        self.progress.push((0, 0));
        self.children.push(element.clone());
        element.inject(|w| w.component(NoRenderRoot));
    }

    fn undraw_child(&mut self, element: &Arc<Widget>) {
        if let Some(i) = self.children.iter().position(|v| Arc::ptr_eq(v, element)) {
            self.children.remove(i);
            self.progress.remove(i);
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

impl ElementRenderer for VelocityRenderer<'_> {
    fn before_draw(&mut self, scope: &mut crate::prelude::RenderScope, widget: &Arc<Widget>) {
        let t = scope.get_transform_mut();

        if let Some(Velocity(vx, vy)) = widget.get() {
            if let Some((ax, ay)) = self.1.get_mut(self.2) {
                add_i32_to_u16(&mut t.x, *ax);
                add_i32_to_u16(&mut t.y, *ay);

                if apply_velocity(self.3, vx, &mut t.x) {
                    apply_value(ax, vx);
                }
                if apply_velocity(self.3, vy, &mut t.y) {
                    apply_value(ay, vy);
                }
            } else {
                self.1.push((0, 0));
            }
        }

        self.2 += 1;

        self.0.width = self.0.width.max(t.x + t.width + (t.px * 2));
        self.0.height = self.0.height.max(t.y + t.height + (t.py * 2));

        t.x += self.0.x + self.0.px;
        t.y += self.0.y + self.0.py;
        t.offset_y = self.0.offset_y;
    }
}

fn apply_velocity(ticks: u16, velocity: i32, x: &mut u16) -> bool {
    let abs_v = velocity.abs();
    if abs_v == 0 {
        return false;
    }

    let interval = (1000 / abs_v).max(1); // prevent divide-by-zero
    if ticks as i32 % interval == 0 {
        if velocity > 0 {
            *x = x.saturating_add(1);
        } else {
            *x = x.saturating_sub(1);
        }
        return true;
    }
    false
}

fn add_i32_to_u16(value: &mut u16, delta: i32) {
    if delta >= 0 {
        *value = value.saturating_add(delta as u16);
    } else {
        *value = value.saturating_sub((-delta) as u16);
    }
}

fn apply_value(x: &mut i32, velocity: i32) {
    *x = if velocity > 0 {
        x.saturating_add(1)
    } else {
        x.saturating_sub(1)
    };
}

component!(Velocity(pub i32, pub i32));
