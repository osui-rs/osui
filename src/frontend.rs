use std::sync::Arc;

use crate::{
    dependency::{Dependency, DependencyHandler},
    widget::{Element, Widget},
    Screen,
};

pub struct Rsx(
    pub  Vec<(
        Box<dyn FnMut() -> Box<dyn Element> + Send + Sync>,
        Vec<std::sync::Arc<dyn DependencyHandler>>,
    )>,
);

impl Rsx {
    pub fn draw(self, screen: &Arc<Screen>) {
        for i in self.0 {
            let d = Dependency::from(i.0, i.1);
            screen.draw_widget(Widget::new(Box::new(d)));
        }
    }
}
