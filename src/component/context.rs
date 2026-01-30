use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    component::EventHandler,
    engine::{Command, CommandExecutor},
    render::DrawContext,
    state::{use_effect, HookDependency},
    View,
};

use super::{scope::Scope, Component, ComponentImpl};

pub struct Context {
    component: Mutex<Component>,
    event_handlers: Mutex<HashMap<TypeId, Vec<EventHandler>>>,
    view: Mutex<View>,
    pub(crate) scopes: Mutex<Vec<Arc<Scope>>>,
    executor: Arc<dyn CommandExecutor>,
}

impl Context {
    pub fn new<F: ComponentImpl + 'static>(
        component: F,
        executor: Arc<dyn CommandExecutor>,
    ) -> Arc<Self> {
        Arc::new(Self {
            component: Mutex::new(Arc::new(component)),
            event_handlers: Mutex::new(HashMap::new()),
            view: Mutex::new(Arc::new(|_| {})),
            scopes: Mutex::new(Vec::new()),
            executor,
        })
    }

    pub fn refresh(self: &Arc<Self>) {
        let s = self.clone();

        std::thread::spawn({
            move || {
                s.event_handlers.lock().unwrap().clear();
                let c = s.component.lock().unwrap().clone();
                *s.view.lock().unwrap() = c.call(&s);
            }
        });
    }

    pub fn refresh_atomic(self: &Arc<Self>) -> Arc<Mutex<bool>> {
        let s = self.clone();
        let done = Arc::new(Mutex::new(false));

        std::thread::spawn({
            let done = done.clone();
            move || {
                s.event_handlers.lock().unwrap().clear();
                let c = s.component.lock().unwrap().clone();
                *s.view.lock().unwrap() = c.call(&s);
                *done.lock().unwrap() = true;
            }
        });

        done
    }

    pub fn get_view(self: &Arc<Self>) -> View {
        self.view.lock().unwrap().clone()
    }

    pub fn on_event<T: Any + 'static, F: Fn(&Arc<Self>, &T) + Send + Sync + 'static>(
        self: &Arc<Self>,
        handler: F,
    ) {
        self.event_handlers
            .lock()
            .unwrap()
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Vec::new())
            .push(Arc::new(Mutex::new(
                move |ctx: &Arc<Self>, event: &dyn Any| {
                    if let Some(e) = event.downcast_ref::<T>() {
                        (handler)(ctx, e);
                    }
                },
            )));
    }

    pub fn emit_event<E: Any + 'static>(self: &Arc<Self>, event: &E) {
        if let Some(v) = self.event_handlers.lock().unwrap().get(&TypeId::of::<E>()) {
            for i in v {
                (i.lock().unwrap())(self, event);
            }
        }

        for scope in self.scopes.lock().unwrap().iter() {
            for (child, _) in scope.children.lock().unwrap().iter() {
                child.emit_event(event);
            }
        }
    }

    pub fn emit_event_threaded<E: Any + Send + Sync + Clone + 'static>(
        self: &Arc<Self>,
        event: &E,
    ) {
        if let Some(v) = self.event_handlers.lock().unwrap().get(&TypeId::of::<E>()) {
            for i in v {
                let i = i.clone();
                let event = event.clone();
                let s = self.clone();
                std::thread::spawn(move || {
                    (i.lock().unwrap())(&s, &event);
                });
            }
        }

        for scope in self.scopes.lock().unwrap().iter() {
            for (child, _) in scope.children.lock().unwrap().iter() {
                child.emit_event_threaded(event);
            }
        }
    }

    pub fn scope(self: &Arc<Self>) -> Arc<Scope> {
        let scope = Scope::new(self.executor.clone());
        self.scopes.lock().unwrap().push(scope.clone());

        scope
    }

    pub fn dyn_scope<F: Fn(&Arc<Scope>) + Send + Sync + 'static>(
        self: &Arc<Self>,
        drawer: F,
        dependencies: &[&dyn HookDependency],
    ) -> Arc<Scope> {
        let scope = Scope::new(self.executor.clone());
        self.scopes.lock().unwrap().push(scope.clone());

        drawer(&scope);

        use_effect(
            {
                let scope = scope.clone();
                move || {
                    drawer(&scope);
                }
            },
            dependencies,
        );

        scope
    }

    pub fn draw_children(self: &Arc<Self>, ctx: &mut DrawContext) {
        for scope in self.scopes.lock().unwrap().iter() {
            for (child, view_wrapper) in scope.children.lock().unwrap().iter() {
                let view = child.get_view();

                if let Some(view_wrapper) = view_wrapper {
                    view_wrapper(ctx, view)
                } else {
                    ctx.draw_view(ctx.area.clone(), view);
                }
            }
        }
    }

    pub fn get_executor(self: &Arc<Self>) -> Arc<dyn CommandExecutor> {
        self.executor.clone()
    }

    pub fn execute<T: Command + 'static>(self: &Arc<Self>, command: T) -> crate::Result<()> {
        self.executor
            .execute_command(&(Arc::new(command) as Arc<dyn Command>))
    }

    pub fn stop(self: &Arc<Self>) -> crate::Result<()> {
        self.execute(crate::engine::commands::Stop)
    }
}
