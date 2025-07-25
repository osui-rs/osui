use crate::{render_scope::RenderScope, widget::Element};
use std::sync::{Arc, Mutex};

pub struct State<T> {
    pub value: *mut T,
    dependents: Mutex<Vec<Arc<Mutex<StateDependency>>>>,
}

pub fn use_state<T>(mut v: T) -> Arc<State<T>> {
    Arc::new(State {
        value: &mut v,
        dependents: Mutex::new(Vec::new()),
    })
}

impl<T> State<T> {
    pub fn set(&self, v: T) {
        unsafe {
            *self.value = v;
        }

        for d in self.dependents.lock().unwrap().iter() {
            let mut d = d.lock().unwrap();
            d.0 = (d.1)();
        }
    }

    pub fn draw<F: FnMut() -> Box<dyn Element> + 'static + Send + Sync>(
        self: &Arc<Self>,
        mut element: F,
    ) -> Arc<Mutex<StateDependency>> {
        let d = Arc::new(Mutex::new(StateDependency((element)(), Box::new(element))));
        self.dependents.lock().unwrap().push(d.clone());
        d
    }
}

pub struct StateDependency(
    Box<dyn Element>,
    pub Box<dyn FnMut() -> Box<dyn Element> + Send + Sync>,
);

impl<'a> Element for Arc<Mutex<StateDependency>> {
    fn render(&mut self, scope: &mut RenderScope) {
        self.lock().unwrap().0.render(scope);
    }

    fn after_render(&mut self, scope: &RenderScope) {
        self.lock().unwrap().0.after_render(scope);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl<T: std::fmt::Display> std::fmt::Display for State<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe { &*self.value }.fmt(f)
    }
}

unsafe impl<T> Send for State<T> {}
unsafe impl<T> Sync for State<T> {}
