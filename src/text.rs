use crate::{components, Component, Element};
use std::{any::TypeId, collections::HashMap};

#[derive(Debug)]
pub struct DisplayComponent();
impl Component for DisplayComponent {}

pub struct Text {
    text: String,
    components: HashMap<TypeId, Box<dyn Component>>,
}

impl Element for Text {
    fn get_component<T: Component + 'static>(&mut self) -> Option<&mut T> {
        self.components
            .get_mut(&TypeId::of::<T>())?
            .downcast_mut::<T>()
    }

    fn add_component<T: Component + 'static>(&mut self, component: T) {
        if !self.components.contains_key(&TypeId::of::<T>()) {
            self.components
                .insert(TypeId::of::<T>(), Box::new(component));
        }
    }
}

pub fn text(text: &str) -> Text {
    Text {
        text: text.to_string(),
        components: components!(DisplayComponent: DisplayComponent()),
    }
}
