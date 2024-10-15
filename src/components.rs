use crate::{Component, ComponentParams};

#[derive(Debug)]
pub struct Div {
    pub test: i32,
    pub children: Vec<Box<dyn Component>>,
}

impl Component for Div {}

pub fn div(params: ComponentParams) -> Div {
    Div {
        test: 0,
        children: params.children,
    }
}
