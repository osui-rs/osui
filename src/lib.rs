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
    String(Arc<dyn Fn() -> String + Send + Sync>),
    Component(component::Component),
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Component(_) => format!("Component"),
                Self::String(v) => format!("String({})", v()),
            }
        )
    }
}
