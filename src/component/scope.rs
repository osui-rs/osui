use std::sync::{Arc, Mutex};

use crate::{engine::CommandExecutor, View, ViewWrapper};

use super::{context::Context, ComponentImpl};

pub struct Scope {
    pub children: Mutex<Vec<(Arc<Context>, Option<ViewWrapper>)>>,
    executor: Arc<dyn CommandExecutor>,
}

impl Scope {
    pub fn new(executor: Arc<dyn CommandExecutor>) -> Arc<Self> {
        Arc::new(Self {
            children: Mutex::new(Vec::new()),
            executor,
        })
    }

    pub fn child<F: ComponentImpl + 'static>(
        self: &Arc<Self>,
        child: F,
        view_wrapper: Option<ViewWrapper>,
    ) {
        let ctx = Context::new(child, self.executor.clone());

        ctx.refresh();

        self.children.lock().unwrap().push((ctx, view_wrapper));
    }

    pub fn view(self: &Arc<Self>, view: View) {
        let ctx = Context::new(view, self.executor.clone());

        ctx.refresh();

        self.children.lock().unwrap().push((ctx, None));
    }
}
