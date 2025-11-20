use crate::renderer::{OsuiRenderer, Renderer};

pub mod error;
pub mod renderer;

pub use error::Result;

pub type Node = Box<dyn Fn(&mut dyn Renderer)>;
pub type Component = Box<dyn Fn(&mut Context) -> Node>;

pub struct Context {}

pub struct Osui {
    node: Node,
    renderer: Option<Box<dyn Renderer>>,
}

impl Osui {
    pub fn new(component: Component) -> Self {
        let mut cx = Context {};
        Osui {
            node: component(&mut cx),
            renderer: None,
        }
    }
    pub fn with_renderer(component: Component, renderer: Box<dyn Renderer>) -> Self {
        let mut cx = Context {};
        Osui {
            node: component(&mut cx),
            renderer: Some(renderer),
        }
    }

    pub fn render(&mut self) -> Result<()> {
        renderer::utils::clear()?;
        if let Some(renderer) = &mut self.renderer {
            (self.node)(renderer.as_mut());
        } else {
            (self.node)(&mut OsuiRenderer);
        }
        Ok(())
    }
}
