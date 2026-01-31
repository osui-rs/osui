use std::sync::Arc;

use access_cell::AccessCell;

use crate::{engine::CommandExecutor, View, ViewWrapper};

use super::{context::Context, ComponentImpl};

pub struct Scope {
    pub children: AccessCell<Vec<(Arc<Context>, Option<ViewWrapper>)>>,
    executor: Arc<dyn CommandExecutor>,
}

impl Scope {
    pub fn new(executor: Arc<dyn CommandExecutor>) -> Arc<Self> {
        Arc::new(Self {
            children: AccessCell::new(Vec::new()),
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

        self.children
            .access(|children| children.push((ctx, view_wrapper)));
    }

    pub fn view(self: &Arc<Self>, view: View) {
        let ctx = Context::new(view, self.executor.clone());

        ctx.refresh();

        self.children.access(|children| children.push((ctx, None)));
    }
}
