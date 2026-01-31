use super::{context::Context, ComponentImpl};
use crate::{engine::CommandExecutor, View, ViewWrapper};
use access_cell::AccessCell;
use std::sync::Arc;

pub type ScopeChildren = Vec<(Arc<Context>, Option<ViewWrapper>)>;

pub struct Scope {
    children: AccessCell<ScopeChildren>,
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

    pub fn view_wrapper(self: &Arc<Self>, view: View, view_wrapper: Option<ViewWrapper>) {
        let ctx = Context::new(view, self.executor.clone());

        ctx.refresh();

        self.children
            .access(|children| children.push((ctx, view_wrapper)));
    }

    pub fn access_children(self: &Arc<Self>, f: impl FnOnce(&mut ScopeChildren) + Send + 'static) {
        self.children.access(f);
    }
}
