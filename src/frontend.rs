use std::sync::Arc;

use crate::{
    component::{Context, Scope},
    state::HookDependency,
    View,
};

pub enum RsxScope {
    Static(Box<dyn Fn(&Arc<Scope>) + Send + Sync>),
    Dynamic(
        Arc<dyn Fn(&Arc<Scope>) + Send + Sync>,
        Vec<Box<dyn HookDependency>>,
    ),
    Child(Rsx),
}

pub struct Rsx(Vec<RsxScope>);

impl Rsx {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn static_scope<F: Fn(&Arc<Scope>) + Send + Sync + 'static>(&mut self, scope: F) {
        self.0.push(RsxScope::Static(Box::new(scope)));
    }

    pub fn dynamic_scope<F: Fn(&Arc<Scope>) + Send + Sync + 'static>(
        &mut self,
        drawer: F,
        dependencies: Vec<Box<dyn HookDependency>>,
    ) {
        self.0
            .push(RsxScope::Dynamic(Arc::new(drawer), dependencies));
    }

    pub fn generate_children(&self, context: &Arc<Context>) {
        let executor = context.get_executor();

        for scope in &self.0 {
            match scope {
                RsxScope::Static(scope_fn) => {
                    let scope = Scope::new(executor.clone());
                    (scope_fn)(&scope);
                    context.scopes.lock().unwrap().push(scope)
                }
                RsxScope::Dynamic(drawer, dependencies) => {
                    let drawer = drawer.clone();
                    context.dyn_scope(
                        move |c| drawer(c),
                        &dependencies.iter().map(|d| d.as_ref()).collect::<Vec<_>>(),
                    );
                }
                RsxScope::Child(child) => child.generate_children(context),
            }
        }
    }

    pub fn view(&self, context: &Arc<Context>) -> View {
        let context = context.clone();

        self.generate_children(&context);

        Arc::new({
            move |ctx| {
                context.draw_children(ctx);
            }
        })
    }
}
