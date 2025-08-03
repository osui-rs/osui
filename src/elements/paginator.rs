use std::sync::Arc;

use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    widget::{Element, Widget},
    NoRender, NoRenderRoot,
};

pub struct Paginator {
    children: Vec<Arc<Widget>>,
    size: (u16, u16),
    index: usize,
}

impl Paginator {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            size: (0, 0),
            index: 0,
        }
    }
}

impl Element for Paginator {
    fn render(&mut self, scope: &mut crate::render_scope::RenderScope) {
        let (width, height) = scope.get_size_or(self.size.0, self.size.1);
        scope.use_area(width, height);
    }

    fn after_render(&mut self, scope: &mut crate::render_scope::RenderScope) {
        if let Some(elem) = self.children.get(self.index) {
            if elem.get::<NoRender>().is_some() {
                return;
            }

            let mut transform = scope.get_transform().clone();
            let (w, h) = scope.get_parent_size();
            scope.set_parent_size(transform.width, transform.height);
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
            transform.width = transform.width.max(t.x + t.width + (t.px * 2));
            transform.height = transform.height.max(t.y + t.height + (t.py * 2));
            t.x += transform.x + transform.px;
            t.y += transform.y + transform.py;

            scope.draw();
            elem.get_elem().after_render(scope);

            scope.set_parent_size(w, h);
            self.size = (transform.width, transform.height);
        }
    }

    fn event(&mut self, event: &dyn crate::prelude::Event) {
        if let Some(crossterm::event::Event::Key(KeyEvent { code, .. })) = event.get() {
            let cl = self.children.len();
            match code {
                KeyCode::Tab => {
                    if self.index + 1 < cl {
                        self.index += 1;
                    } else {
                        self.index = 0;
                    }
                }
                KeyCode::BackTab => {
                    if self.index > 0 {
                        self.index -= 1;
                    } else if cl > 0 {
                        self.index = cl - 1;
                    }
                }
                _ => {}
            }
        }
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
