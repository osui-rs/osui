use crate::{Component, ComponentParams};

#[derive(Debug)]
pub struct Div {
    pub children: Vec<Box<dyn Component>>,
}

impl Component for Div {}

pub fn div(params: ComponentParams) -> Div {
    Div {
        children: params.children,
    }
}

#[derive(Debug)]
pub struct Text {
    pub text: String,
}

impl Component for Text {}

pub fn text(params: ComponentParams) -> Text {
    Text { text: params.expr }
}
