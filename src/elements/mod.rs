pub mod div;
pub mod grid;
pub mod state;

use crate::Element;

impl Element for String {
    fn render(&mut self, scope: &mut crate::render_scope::RenderScope) {
        scope.draw_text(self);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Element for (String, u32) {
    fn render(&mut self, scope: &mut crate::render_scope::RenderScope) {
        scope.draw_text_colored(&self.0, self.1);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
