use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{render::DrawContext, View, ViewWrapper};

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

pub struct Context {
    component: Mutex<Component>,
    event_handlers: Mutex<HashMap<TypeId, Vec<EventHandler>>>,
    view: Mutex<View>,
    children: Mutex<Vec<(Arc<Context>, Option<ViewWrapper>)>>,
}

impl Context {
    pub fn new<F: Fn(&Arc<Context>) -> View + Send + Sync + 'static>(component: F) -> Arc<Self> {
        Arc::new(Self {
            component: Mutex::new(Arc::new(component)),
            event_handlers: Mutex::new(HashMap::new()),
            view: Mutex::new(Arc::new(|_| {})),
            children: Mutex::new(Vec::new()),
        })
    }

    pub fn refresh(self: &Arc<Context>) {
        let s = self.clone();

        std::thread::spawn({
            move || {
                s.event_handlers.lock().unwrap().clear();
                let c = s.component.lock().unwrap().clone();
                *s.view.lock().unwrap() = (c)(&s);
            }
        });
    }

    pub fn refresh_atomic(self: &Arc<Context>) -> Arc<Mutex<bool>> {
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

    pub fn get_view(self: &Arc<Context>) -> View {
        self.view.lock().unwrap().clone()
    }

    pub fn on_event<T: Event + 'static, F: Fn(&Arc<Context>, &T) + Send + Sync + 'static>(
        self: &Arc<Context>,
        handler: F,
    ) {
        self.event_handlers
            .lock()
            .unwrap()
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Vec::new())
            .push(Arc::new(Mutex::new(
                move |ctx: &Arc<Context>, event: &dyn Event| {
                    if let Some(e) = event.get::<T>() {
                        (handler)(ctx, e);
                    }
                },
            )));
    }

    pub fn emit_event<E: Event + 'static>(self: &Arc<Context>, event: &E) {
        if let Some(v) = self.event_handlers.lock().unwrap().get(&TypeId::of::<E>()) {
            for i in v {
                (i.lock().unwrap())(self, event);
            }
        }

        for (child, _) in self.children.lock().unwrap().iter() {
            child.emit_event(event);
        }
    }

    pub fn emit_event_threaded<E: Event + Send + Sync + Clone + 'static>(
        self: &Arc<Context>,
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

        for (child, _) in self.children.lock().unwrap().iter() {
            child.emit_event_threaded(event);
        }
    }

    pub fn child<F: Fn(&Arc<Context>) -> View + Send + Sync + 'static>(
        self: &Arc<Context>,
        child: F,
        view_wrapper: Option<ViewWrapper>,
    ) {
        let ctx = Context::new(child);

        ctx.refresh();

        self.children.lock().unwrap().push((ctx, view_wrapper));
    }

    pub fn draw_children(self: &Arc<Context>, ctx: &mut DrawContext) {
        for (child, view_wrapper) in self.children.lock().unwrap().iter() {
            let view = child.get_view();

            if let Some(view_wrapper) = view_wrapper {
                view_wrapper(ctx, view)
            } else {
                ctx.draw_view(ctx.allocated.clone(), view);
            }
        }
    }
}
