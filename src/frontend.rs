use std::sync::Arc;

use crate::{
    component::{Context, Scope},
    state::HookDependency,
    View,
};

pub enum RsxScope {
    Static(Arc<Scope>),
    Dynamic(
        Arc<dyn Fn(&Arc<Scope>) + Send + Sync>,
        Vec<Box<dyn HookDependency>>,
    ),
}

pub struct Rsx(Vec<RsxScope>);

impl Rsx {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn static_scope(&mut self, scope: Arc<Scope>) {
        self.0.push(RsxScope::Static(scope));
    }

    pub fn dynamic_scope<F: Fn(&Arc<Scope>) + Send + Sync + 'static>(
        &mut self,
        drawer: F,
        dependencies: Vec<Box<dyn HookDependency>>,
    ) {
        self.0
            .push(RsxScope::Dynamic(Arc::new(drawer), dependencies));
    }

    pub fn view(&self, context: Arc<Context>) -> View {
        for scope in &self.0 {
            match scope {
                RsxScope::Static(scope) => context.scopes.lock().unwrap().push(scope.clone()),
                RsxScope::Dynamic(drawer, dependencies) => {
                    let drawer = drawer.clone();
                    context.dyn_scope(
                        move |c| drawer(c),
                        &dependencies.iter().map(|d| d.as_ref()).collect::<Vec<_>>(),
                    );
                }
            }
        }

        let context = context.clone();
        Arc::new({
            move |ctx| {
                context.draw_children(ctx);
            }
        })
    }
}
