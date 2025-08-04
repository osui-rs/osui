use std::sync::Arc;

use figlet_rs::FIGfont;

use crate::{
    widget::{Element, Widget},
    NoRender, NoRenderRoot,
};

pub struct Heading {
    pub font: FIGfont,
    pub smooth: bool,
    children: Vec<Arc<Widget>>,
}

impl Heading {
    pub fn new() -> Heading {
        Heading {
            font: FIGfont::standard().unwrap(),
            smooth: false,
            children: Vec::new(),
        }
    }
}

impl Element for Heading {
    fn render(
        &mut self,
        scope: &mut crate::prelude::RenderScope,
        _: &crate::extensions::Context,
    ) {
        let mut s = String::new();
        for element in &self.children {
            if element.get::<NoRender>().is_some() {
                continue;
            }

            if let Some(e) = element.get_elem().as_any().downcast_ref::<String>() {
                s += e;
            }
        }
        if let Some(t) = self.font.convert(&s) {
            scope.draw_text(
                0,
                0,
                &if self.smooth {
                    format!("{t}").replace("-", "─").replace("|", "│")
                } else {
                    format!("{t}")
                },
            );
        }
    }

    fn draw_child(&mut self, element: &Arc<Widget>) {
        element.inject(|w| w.component(NoRenderRoot));
        self.children.push(element.clone());
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
