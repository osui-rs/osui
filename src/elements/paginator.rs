use std::sync::Arc;

use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    prelude::DivRenderer,
    widget::{Element, Widget},
    NoRenderRoot,
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
        if let Some(widget) = self.children.get(self.index) {
            let mut transform = scope.get_transform().clone();
            let mut renderer = DivRenderer(&mut transform);
            let (w, h) = scope.get_parent_size();
            scope.set_parent_size(renderer.0.width, renderer.0.height);

            scope.render_widget(&mut renderer, render_context.get_context(), widget);

            scope.set_parent_size(w, h);
            self.size = (renderer.0.width, renderer.0.height);
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
