use crate::renderer::{OsuiRenderer, Renderer};

pub mod error;
pub mod renderer;

pub use error::Result;

pub trait Component {
    fn render(&mut self, renderer: &mut dyn Renderer);
}

pub struct Osui {
    component: Box<dyn Component>,
    renderer: Option<Box<dyn Renderer>>,
}

impl Osui {
    pub fn new(component: Box<dyn Component>) -> Self {
        Osui {
            component,
            renderer: None,
        }
    }
    pub fn with_renderer(component: Box<dyn Component>, renderer: Box<dyn Renderer>) -> Self {
        Osui {
            component,
            renderer: Some(renderer),
        }
    }

    pub fn render(&mut self) {
        if let Some(renderer) = &mut self.renderer {
            self.component.render(renderer.as_mut());
        } else {
            self.component.render(&mut OsuiRenderer);
        }
    }
}
