use std::fmt::Debug;

pub mod components;
pub mod macros;

pub trait Component: Debug {
    fn render(&mut self) -> String {
        String::new()
    }
}

pub struct ComponentParams {
    pub children: Vec<Box<dyn Component>>,
    pub expr: String,
}
