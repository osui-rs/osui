pub mod div;
pub mod flex;
pub mod heading;
pub mod input;
pub mod paginator;

pub use div::*;
pub use flex::*;
pub use heading::*;
pub use input::*;
pub use paginator::*;

use crate::Element;

impl Element for String {
    fn render(
        &mut self,
        scope: &mut crate::render_scope::RenderScope,
        _: &crate::render_scope::RenderContext,
    ) {
        scope.draw_text(0, 0, self);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
