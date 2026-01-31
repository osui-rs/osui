use std::sync::Arc;

use crate::component::{context::Context, scope::Scope};
use crate::{render::Point, state::HookDependency, View};

pub trait ToRsx {
    fn to_rsx(&self) -> Rsx;
}

#[derive(Clone)]
pub enum RsxScope {
    Static(Arc<dyn Fn(&Arc<Scope>) + Send + Sync>),
    Dynamic(
        Arc<dyn Fn(&Arc<Scope>) + Send + Sync>,
        Vec<Arc<dyn HookDependency>>,
    ),
    Child(Rsx),
}

#[derive(Clone)]
pub struct Rsx(Vec<RsxScope>);

impl Rsx {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn static_scope<F: Fn(&Arc<Scope>) + Send + Sync + 'static>(&mut self, scope: F) {
        self.0.push(RsxScope::Static(Arc::new(scope)));
    }

    pub fn dynamic_scope<F: Fn(&Arc<Scope>) + Send + Sync + 'static>(
        &mut self,
        drawer: F,
        dependencies: Vec<Arc<dyn HookDependency>>,
    ) {
        self.0
            .push(RsxScope::Dynamic(Arc::new(drawer), dependencies));
    }

    pub fn child<R: ToRsx>(&mut self, child: R) {
        self.0.push(RsxScope::Child(child.to_rsx()));
    }

    pub fn generate_children(&self, context: &Arc<Context>) {
        let executor = context.get_executor();

        for scope in &self.0 {
            match scope {
                RsxScope::Static(scope_fn) => {
                    let scope = Scope::new(executor.clone());
                    (scope_fn)(&scope);
                    context.add_scope(scope)
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

impl ToRsx for &Rsx {
    fn to_rsx(&self) -> Rsx {
        Rsx(self.0.clone())
    }
}

impl<T: std::fmt::Display> ToRsx for T {
    fn to_rsx(&self) -> Rsx {
        let s = self.to_string();
        Rsx(vec![RsxScope::Static(Arc::new(move |scope| {
            let s = s.clone();
            scope.view(Arc::new(move |ctx| ctx.draw_text(Point { x: 0, y: 0 }, &s)))
        }))])
    }
}
