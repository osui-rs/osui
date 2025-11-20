use std::sync::Arc;

pub mod component;
pub mod state;

pub mod prelude {
    pub use crate::component::*;
    pub use crate::state::*;
    pub use crate::Node;
}

#[derive(Clone)]
pub enum Node {
    String(Arc<dyn Fn() -> String>),
    Component(component::Component),
}
