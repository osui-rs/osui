use std::sync::Arc;

use crate::{state::DependencyHandler, widget::WidgetLoad, Screen};

pub struct Rsx(
    pub  Vec<(
        Box<dyn FnMut() -> WidgetLoad + Send + Sync>,
        Vec<Box<dyn DependencyHandler>>,
    )>,
);

impl Rsx {
    pub fn draw(self, screen: &Arc<Screen>) {
        for i in self.0 {
            let w = screen.draw_box(i.0);
            for d in i.1 {
                w.dependency_box(d);
            }
        }
    }

    pub fn create_element<F: FnMut() -> WidgetLoad + Send + Sync + 'static>(
        &mut self,
        load: F,
        dependencies: Vec<Box<dyn DependencyHandler>>,
    ) {
        self.0.push((Box::new(load), dependencies));
    }
}
