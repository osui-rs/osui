use std::sync::Arc;

use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    prelude::DivRenderer,
    widget::{Element, Widget},
    NoRenderRoot,
};

pub struct Paginator {
    children: Vec<Arc<Widget>>,
    index: usize,
}

impl Paginator {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            index: 0,
        }
    }
}

impl Element for Paginator {
    fn render(
        &mut self,
        parent_scope: &mut crate::render_scope::RenderScope,
        render_context: &crate::render_scope::RenderContext,
    ) {
        if let Some(widget) = self.children.get(self.index) {
            let mut transform = parent_scope.get_transform().clone();
            let (w, h) = parent_scope.get_size_or_parent();

            let mut scope = crate::render_scope::RenderScope::new();
            scope.set_parent_size(w, h);

            let mut renderer = DivRenderer(&mut transform);
            scope.render_widget(&mut renderer, render_context.get_context(), widget);

            parent_scope.use_area(w, h);
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

    fn is_ghost(&mut self) -> bool {
        true
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
