use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, Mutex},
};

use access_cell::AccessCell;

use crate::{
    component::EventHandler,
    engine::{Command, CommandExecutor},
    render::DrawContext,
    state::{use_effect, HookDependency},
    View,
};

use super::{scope::Scope, Component, ComponentImpl};

pub struct Context {
    component: AccessCell<Component>,
    view: AccessCell<View>,
    event_handlers: AccessCell<HashMap<TypeId, Vec<EventHandler>>>,
    scopes: AccessCell<Vec<Arc<Scope>>>,
    executor: Arc<dyn CommandExecutor>,
}

impl Context {
    pub fn new<F: ComponentImpl + 'static>(
        component: F,
        executor: Arc<dyn CommandExecutor>,
    ) -> Arc<Self> {
        Arc::new(Self {
            component: AccessCell::new(Arc::new(component)),
            view: AccessCell::new(Arc::new(|_| {})),
            event_handlers: AccessCell::new(HashMap::new()),
            scopes: AccessCell::new(Vec::new()),
            executor,
        })
    }

    pub fn refresh(self: &Arc<Self>) {
        self.event_handlers
            .access(|event_handlers| event_handlers.clear());
        self.component.access({
            let s = self.clone();
            move |component| {
                let component = component.clone();
                s.view.access({
                    let s = s.clone();
                    move |view| *view = component.call(&s)
                })
            }
        });
    }

    pub fn refresh_sync(self: &Arc<Self>) {
        let (tx, rx) = std::sync::mpsc::channel::<()>();

        self.event_handlers
            .access(|event_handlers| event_handlers.clear());

        self.component.access({
            let s = self.clone();
            move |component| {
                let component = component.clone();

                s.view.access({
                    let s = s.clone();
                    let tx = tx.clone();
                    move |view| {
                        *view = component.call(&s);
                        let _ = tx.send(()); // signal completion
                    }
                });
            }
        });

        // BLOCK until view closure finishes
        let _ = rx.recv();
    }

    pub fn get_view(self: &Arc<Self>) -> View {
        self.view.access_ref().clone()
    }

    pub fn on_event<T: Any + 'static, F: Fn(&Arc<Self>, &T) + Send + Sync + 'static>(
        self: &Arc<Self>,
        handler: F,
    ) {
        let new_handler: EventHandler =
            Arc::new(Mutex::new(move |ctx: &Arc<Context>, event: &dyn Any| {
                if let Some(e) = event.downcast_ref::<T>() {
                    (handler)(ctx, e);
                }
            }));
        self.event_handlers.access(|event_handlers| {
            event_handlers
                .entry(TypeId::of::<T>())
                .or_insert_with(Vec::new)
                .push(new_handler);
        });
    }

    pub fn emit_event<E: Send + Sync + Any + 'static>(self: &Arc<Self>, event: E) {
        let event = Arc::new(event);
        let handlers_to_call: Vec<EventHandler> = {
            let guard = self.event_handlers.access_ref();
            guard.get(&TypeId::of::<E>()).cloned().unwrap_or_default()
        };
        for h in &handlers_to_call {
            (h.lock().unwrap())(self, event.as_ref());
        }

        self.scopes.access(move |scopes| {
            for scope in scopes {
                for (child, _) in scope.children.lock().unwrap().iter() {
                    child.emit_event(event.clone());
                }
            }
        });
    }

    pub fn emit_event_threaded<E: Any + Send + Sync + Clone + 'static>(
        self: &Arc<Self>,
        event: &E,
    ) {
        let handlers_to_call: Vec<EventHandler> = {
            let guard = self.event_handlers.access_ref();
            guard.get(&TypeId::of::<E>()).cloned().unwrap_or_default()
        };
        for h in handlers_to_call {
            let event = event.clone();
            let s = self.clone();
            std::thread::spawn(move || {
                (h.lock().unwrap())(&s, &event);
            });
        }

        let event = event.clone();
        self.scopes.access(move |scopes| {
            for scope in scopes {
                for (child, _) in scope.children.lock().unwrap().iter() {
                    child.emit_event_threaded(&event);
                }
            }
        });
    }

    pub fn scope(self: &Arc<Self>) -> Arc<Scope> {
        let scope = Scope::new(self.executor.clone());

        self.scopes.access({
            let scope = scope.clone();
            move |scopes| scopes.push(scope)
        });

        scope
    }

    pub fn dyn_scope<F: Fn(&Arc<Scope>) + Send + Sync + 'static>(
        self: &Arc<Self>,
        drawer: F,
        dependencies: &[&dyn HookDependency],
    ) -> Arc<Scope> {
        let scope = Scope::new(self.executor.clone());

        self.scopes.access({
            let scope = scope.clone();
            move |scopes| scopes.push(scope)
        });

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

    pub fn add_scope(self: &Arc<Self>, scope: Arc<Scope>) {
        self.scopes.access(|scopes| scopes.push(scope));
    }

    pub fn draw_children(self: &Arc<Self>, ctx: &mut DrawContext) {
        let mut c = ctx.clone();

        let (tx, rx) = std::sync::mpsc::channel::<DrawContext>();

        self.scopes.access(move |scopes| {
            for scope in scopes {
                for (child, view_wrapper) in scope.children.lock().unwrap().iter() {
                    let view = child.get_view();

                    if let Some(view_wrapper) = view_wrapper {
                        view_wrapper(&mut c, view)
                    } else {
                        c.draw_view(c.area.clone(), view);
                    }

                    tx.send(c.clone()).expect("Failed transmitting DrawContext");
                }
            }
        });

        *ctx = rx.recv().expect("Failed receiving DrawContext");
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
