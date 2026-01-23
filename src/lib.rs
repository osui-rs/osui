use std::sync::Arc;

pub mod component;
pub mod state;

pub mod prelude {
    pub use crate::component::*;
    pub use crate::state::*;
    pub use crate::{DrawContext, View};
}

pub type View = Arc<dyn Fn(&mut DrawContext) + Send + Sync>;

pub struct DrawContext {}
