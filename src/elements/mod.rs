pub mod div;
pub mod flex;
pub mod heading;
pub mod input;

pub use div::*;
pub use flex::*;
pub use heading::*;
pub use input::*;

use crate::Element;

impl Element for String {
    fn render(&mut self, scope: &mut crate::render_scope::RenderScope) {
        scope.draw_text(0, 0, self);
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
        scope.draw_text_colored(0, 0, &self.0, self.1);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
