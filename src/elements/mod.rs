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
