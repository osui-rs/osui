use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::{
    events::EventManager,
    extensions::{Extension, ExtensionManager},
    render_scope::RenderScope,
    style::Transform,
};

pub mod events;
pub mod extensions;
pub mod macros;
pub mod render_scope;
pub mod style;
pub mod text;
pub mod utils;

pub trait Element {
    #[allow(unused)]
    fn render(&mut self, scope: &mut RenderScope) {}
    #[allow(unused)]
    fn init(&mut self, events: &mut EventManager) {}
    fn as_any(self: Box<Self>) -> Box<dyn Any>;
}

pub trait Component {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct Widget(pub Box<dyn Element>, HashMap<TypeId, Box<dyn Component>>);

impl Widget {
    pub fn component<C: Component + 'static>(&mut self, c: C) -> &mut Self {
        self.1.entry(c.type_id()).or_insert_with(|| Box::new(c));
        self
    }

    pub fn set_component<C: Component + 'static>(&mut self, c: C) -> &mut Self {
        self.1.insert(c.type_id(), Box::new(c));
        self
    }

    pub fn get<C: Component + 'static>(&self) -> Option<&C> {
        self.1
            .get(&TypeId::of::<C>())
            .and_then(|c| c.as_any().downcast_ref::<C>())
    }

    pub fn get_mut<C: Component + 'static>(&mut self) -> Option<&mut C> {
        self.1
            .get_mut(&TypeId::of::<C>())
            .and_then(|c| c.as_any_mut().downcast_mut::<C>())
    }
}

pub struct Screen {
    pub elements: Vec<Widget>,
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            elements: Vec::new(),
        }
    }

    pub fn draw<E: Element + 'static>(&mut self, element: E) -> &mut Widget {
        self.elements
            .push(Widget(Box::new(element), HashMap::new()));
        self.elements.last_mut().unwrap()
    }

    pub fn run(
        &mut self,
        events: &mut EventManager,
        extensions: &mut ExtensionManager,
    ) -> std::io::Result<()> {
        let mut scope = RenderScope::new();

        for elem in &mut self.elements {
            elem.component(Transform::new());
            elem.0.init(events);
        }

        extensions.init(self, events);

        loop {
            extensions.tick_start(self, events);

            utils::clear().unwrap();
            for elem in &mut self.elements {
                scope.clear();
                elem.0.render(&mut scope);
                if let Some(t) = elem.get() {
                    scope.set_transform(t);
                }
                scope.draw();
            }

            extensions.tick_end(self, events);
        }
    }
}
