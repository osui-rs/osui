use std::{any::TypeId, fmt::Debug};

use crate::{ElementComponent, Extension};

pub struct OutputExtension;

impl Extension for OutputExtension {
    fn tick(&mut self, args: &mut crate::Args) {
        for elem in &mut args.base_elements {
            for comp in elem.get_components() {
                if comp.get_type_id() == TypeId::of::<RenderAble>() {
                    comp
                }
            }
        }
    }
}

pub struct RenderAble(pub Box<dyn Fn() -> String>);

impl ElementComponent for RenderAble {
    fn get_type_id(&self) -> std::any::TypeId {
        TypeId::of::<RenderAble>()
    }
}

impl RenderAble {
    pub fn new<F: Fn() -> String + 'static>(f: F) -> Box<RenderAble> {
        Box::new(RenderAble(Box::new(f)))
    }
}
