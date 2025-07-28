use std::sync::Arc;

use crate::{state::DependencyHandler, widget::WidgetLoad, Screen};

pub struct Rsx(
    pub  Vec<(
        Box<dyn FnMut() -> WidgetLoad + Send + Sync>,
        Vec<std::sync::Arc<dyn DependencyHandler>>,
    )>,
);

impl Rsx {
    pub fn draw(self, screen: &Arc<Screen>) {
        for i in self.0 {
            screen.draw_box(i.0);
        }
    }
}
