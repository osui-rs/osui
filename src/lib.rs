use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub mod text;

pub trait Event: Any {}

pub struct Screen {
    element: Box<dyn Element>,
    event_handlers: HashMap<TypeId, Box<dyn FnMut(Box<dyn Event>)>>,
}

impl Screen {
    
}

pub trait Element {
    fn init(&mut self, screen: &mut Screen);
}
