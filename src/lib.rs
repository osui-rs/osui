use std::sync::Arc;

use crate::render::DrawContext;

pub mod component;
pub mod engine;
pub mod frontend;
pub mod render;
pub mod state;

pub mod prelude {
    pub use crate::component::{context::*, scope::*, *};
    pub use crate::engine::*;
    pub use crate::frontend::*;
    pub use crate::render::*;
    pub use crate::state::*;
    pub use crate::{sleep, Error, Result, View, ViewWrapper};
    pub use osui_macros::{component, rsx};
    pub use std::sync::{Arc, Mutex};
}

pub type View = Arc<dyn Fn(&mut DrawContext) + Send + Sync>;
pub type ViewWrapper = Arc<dyn Fn(&mut DrawContext, View) + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    PoisonError,
}

impl From<std::sync::PoisonError<std::sync::MutexGuard<'_, bool>>> for Error {
    fn from(_value: std::sync::PoisonError<std::sync::MutexGuard<'_, bool>>) -> Self {
        Error::PoisonError
    }
}

pub fn sleep(delay_ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(delay_ms));
}
