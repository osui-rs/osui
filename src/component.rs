use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    render::DrawContext,
    state::{use_effect, HookDependency},
    View, ViewWrapper,
};

pub trait Event {
    fn as_any(&self) -> &dyn Any;
}

impl<'a> dyn Event + 'a {
    pub fn get<T: Event + 'static>(&self) -> Option<&T> {
        self.as_any().downcast_ref()
    }
}

pub type Component = Arc<dyn Fn(&Arc<Context>) -> View + Send + Sync>;
pub type EventHandler = Arc<Mutex<dyn FnMut(&Arc<Context>, &dyn Event) + Send + Sync>>;

pub struct Scope {
    pub children: Mutex<Vec<(Arc<Context>, Option<ViewWrapper>)>>,
}

pub struct Context {
    component: Mutex<Component>,
    event_handlers: Mutex<HashMap<TypeId, Vec<EventHandler>>>,
    view: Mutex<View>,
    scopes: Mutex<Vec<Arc<Scope>>>,
}

impl Context {
    pub fn new<F: Fn(&Arc<Self>) -> View + Send + Sync + 'static>(component: F) -> Arc<Self> {
        Arc::new(Self {
            component: Mutex::new(Arc::new(component)),
            event_handlers: Mutex::new(HashMap::new()),
            view: Mutex::new(Arc::new(|_| {})),
            scopes: Mutex::new(Vec::new()),
        })
    }

    pub fn refresh(self: &Arc<Self>) {
        let s = self.clone();

        std::thread::spawn({
            move || {
                s.event_handlers.lock().unwrap().clear();
                let c = s.component.lock().unwrap().clone();
                *s.view.lock().unwrap() = (c)(&s);
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
                *s.view.lock().unwrap() = (c)(&s);
                *done.lock().unwrap() = true;
            }
        });

        done
    }

    pub fn get_view(self: &Arc<Self>) -> View {
        self.view.lock().unwrap().clone()
    }

    pub fn on_event<T: Event + 'static, F: Fn(&Arc<Self>, &T) + Send + Sync + 'static>(
        self: &Arc<Self>,
        handler: F,
    ) {
        self.event_handlers
            .lock()
            .unwrap()
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Vec::new())
            .push(Arc::new(Mutex::new(
                move |ctx: &Arc<Self>, event: &dyn Event| {
                    if let Some(e) = event.get::<T>() {
                        (handler)(ctx, e);
                    }
                },
            )));
    }

    pub fn emit_event<E: Event + 'static>(self: &Arc<Self>, event: &E) {
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

    pub fn emit_event_threaded<E: Event + Send + Sync + Clone + 'static>(
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
        let scope = Arc::new(Scope {
            children: Mutex::new(Vec::new()),
        });
        self.scopes.lock().unwrap().push(scope.clone());

        scope
    }

    pub fn dyn_scope<F: Fn(&Arc<Scope>) + Send + Sync + 'static>(
        self: &Arc<Self>,
        drawer: F,
        dependencies: &[&dyn HookDependency],
    ) -> Arc<Scope> {
        let scope = Arc::new(Scope {
            children: Mutex::new(Vec::new()),
        });
        self.scopes.lock().unwrap().push(scope.clone());

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
}

impl Scope {
    pub fn child<F: Fn(&Arc<Context>) -> View + Send + Sync + 'static>(
        self: &Arc<Self>,
        child: F,
        view_wrapper: Option<ViewWrapper>,
    ) {
        let ctx = Context::new(child);

        ctx.refresh();

        self.children.lock().unwrap().push((ctx, view_wrapper));
    }

    pub fn view(self: &Arc<Self>, view: View) {
        let ctx = Context::new(move |_| view.clone());

        ctx.refresh();

        self.children.lock().unwrap().push((ctx, None));
    }
}
