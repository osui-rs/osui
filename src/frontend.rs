use std::sync::Arc;

use crate::{
    state::DependencyHandler,
    widget::{Widget, WidgetLoad},
    Screen,
};

pub enum RsxElement {
    Element(
        Box<dyn FnMut() -> WidgetLoad + Send + Sync>,
        Vec<Box<dyn DependencyHandler>>,
        Rsx,
    ),
}

pub struct Rsx(pub Vec<RsxElement>);

impl Rsx {
    pub fn draw(self, screen: &Arc<Screen>) {
        self.draw_parent(screen, None);
    }

    pub fn draw_parent(self, screen: &Arc<Screen>, parent: Option<Arc<Widget>>) {
        for rsx_elem in self.0 {
            match rsx_elem {
                RsxElement::Element(f, dep, child) => {
                    let w = screen.draw_box(f);
                    for d in dep {
                        w.dependency_box(d);
                    }
                    child.draw_parent(screen, Some(w.clone()));
                    if let Some(parent) = &parent {
                        parent.0.lock().unwrap().draw_child(&w);
                    }
                }
            }
        }
    }

    pub fn create_element<F: FnMut() -> WidgetLoad + Send + Sync + 'static>(
        &mut self,
        load: F,
        dependencies: Vec<Box<dyn DependencyHandler>>,
        children: Rsx,
    ) {
        self.0
            .push(RsxElement::Element(Box::new(load), dependencies, children));
    }
}
