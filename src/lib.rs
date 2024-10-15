use std::fmt::Debug;

pub mod components;
pub mod macros;

pub trait Component: Debug {}

pub struct ComponentParams {
    pub children: Vec<Box<dyn Component>>,
}
