use crate::Element;

impl Element for String {
    fn render(&mut self, scope: &mut crate::render_scope::RenderScope) {
        scope.draw_text(self);
    }

    fn as_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        Box::new(self)
    }
}
