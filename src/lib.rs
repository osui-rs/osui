use std::sync::Arc;

use crate::render::DrawContext;

pub mod component;
pub mod engine;
pub mod render;
pub mod state;

pub mod prelude {
    pub use crate::component::*;
    pub use crate::engine::*;
    pub use crate::render::*;
    pub use crate::state::*;
    pub use crate::View;
}

pub type View = Arc<dyn Fn(&mut DrawContext) + Send + Sync>;
pub type ViewWrapper = Arc<dyn Fn(&mut DrawContext, View) + Send + Sync>;
