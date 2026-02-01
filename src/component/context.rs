//! # Context Module
//!
//! Provides the Context type which is central to component state management.
//! Context holds component state, manages event handlers, and coordinates
//! rendering and updates across the component tree.

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
    hooks::{use_effect, HookDependency},
    View,
};

use super::{scope::Scope, Component, ComponentImpl};

/// Context represents the runtime state and behavior of a component
/// 
/// Each component instance has a Context that holds:
/// - The component implementation
/// - The current view (render result)
/// - Event handlers for responding to events
/// - Child scopes for managing child components
pub struct Context {
    /// The component implementation
    component: AccessCell<Component>,
    /// The current rendered view
    view: AccessCell<View>,
    /// Event handlers grouped by event type
    event_handlers: AccessCell<HashMap<TypeId, Vec<EventHandler>>>,
    /// Child scopes (component hierarchies)
    pub(crate) scopes: Mutex<Vec<Arc<Scope>>>,
    /// Command executor for this context's command handling
    executor: Arc<dyn CommandExecutor>,
}

impl Context {
    /// Creates a new context for the given component
    pub fn new<F: ComponentImpl + 'static>(
        component: F,
        executor: Arc<dyn CommandExecutor>,
    ) -> Arc<Self> {
        Arc::new(Self {
            component: AccessCell::new(Arc::new(component)),
            view: AccessCell::new(Arc::new(|_| {})),
            event_handlers: AccessCell::new(HashMap::new()),
            scopes: Mutex::new(Vec::new()),
            executor,
        })
    }

    /// Refreshes the component by re-rendering it
    /// 
    /// Clears event handlers and calls the component to produce a new view.
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

    /// Synchronously refreshes the component
    /// 
    /// Blocks until the component has finished rendering.
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

    /// Gets the current view
    pub fn get_view(self: &Arc<Self>) -> View {
        self.view.access_ref().clone()
    }

    /// Registers an event handler for events of type T
    /// 
    /// When an event of type T is emitted, the handler is called with
    /// the context and a reference to the event.
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

    /// Emits an event to this component and all descendants
    /// 
    /// Calls all registered handlers for this event type,
    /// then propagates the event to child components.
    pub fn emit_event<E: Send + Sync + Any + 'static>(self: &Arc<Self>, event: E) {
        let event = Arc::new(event);
        let handlers_to_call: Vec<EventHandler> = {
            let guard = self.event_handlers.access_ref();
            guard.get(&TypeId::of::<E>()).cloned().unwrap_or_default()
        };
        for h in &handlers_to_call {
            (h.lock().unwrap())(self, event.as_ref());
        }

        for scope in self.scopes.lock().unwrap().iter() {
            for (child, _) in scope.children.lock().unwrap().iter() {
                child.emit_event(event.clone());
            }
        }
    }

    /// Emits an event to this component in a spawned thread
    /// 
    /// Similar to emit_event but handlers are called in spawned threads
    /// for concurrent execution.
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

        for scope in self.scopes.lock().unwrap().iter() {
            for (child, _) in scope.children.lock().unwrap().iter() {
                child.emit_event_threaded(event);
            }
        }
    }

    /// Creates a new child scope
    pub fn scope(self: &Arc<Self>) -> Arc<Scope> {
        let scope = Scope::new(self.executor.clone());
        self.scopes.lock().unwrap().push(scope.clone());

        scope
    }

    /// Creates a dynamic child scope that re-renders when dependencies change
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

    /// Adds a pre-constructed scope as a child
    pub fn add_scope(self: &Arc<Self>, scope: Arc<Scope>) {
        self.scopes.lock().unwrap().push(scope);
    }

    /// Renders all child components to the draw context
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

    /// Gets the command executor for this context
    pub fn get_executor(self: &Arc<Self>) -> Arc<dyn CommandExecutor> {
        self.executor.clone()
    }

    /// Executes a command
    pub fn execute<T: Command + 'static>(self: &Arc<Self>, command: T) -> crate::Result<()> {
        self.executor
            .execute_command(&(Arc::new(command) as Arc<dyn Command>))
    }

    /// Stops the application
    pub fn stop(self: &Arc<Self>) -> crate::Result<()> {
        self.execute(crate::engine::commands::Stop)
    }
}
