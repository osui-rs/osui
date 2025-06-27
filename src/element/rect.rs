use crate::Element;

pub struct Rect(pub u32);
impl Element for Rect {
    fn render(&mut self, scope: &mut crate::render_scope::RenderScope) {
        let (w, h) = scope.get_size();

        if w > 0 && h > 0 {
            scope.draw_rect(w, h, self.0);
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
