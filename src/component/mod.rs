pub mod context;
pub mod scope;

use std::{
    any::Any,
    sync::{Arc, Mutex},
};

use crate::View;

use context::Context;

pub type Component = Arc<dyn ComponentImpl>;
pub type EventHandler = Arc<Mutex<dyn FnMut(&Arc<Context>, &dyn Any) + Send + Sync>>;

pub trait ComponentImpl: Send + Sync {
    fn call(&self, cx: &Arc<Context>) -> View;
}

impl ComponentImpl for View {
    fn call(&self, _: &Arc<Context>) -> View {
        self.clone()
    }
}

impl<F> ComponentImpl for F
where
    F: Fn(&Arc<Context>) -> View + Send + Sync,
{
    fn call(&self, ctx: &Arc<Context>) -> View {
        self(ctx)
    }
}
