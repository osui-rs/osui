use std::{any::TypeId, thread, time::Duration};

pub mod output;

pub trait Extension {
    fn tick(&mut self, args: &mut Args);
}

pub trait BaseElement {
    fn init(&mut self);
    fn get_components<'a>(&'a mut self) -> &'a mut Vec<Box<dyn ElementComponent>>;
}

pub trait ElementComponent {
    fn get_type_id(&self) -> TypeId;
}

pub struct Args {
    pub tick_rate: u16,
    pub base_elements: Vec<Box<dyn BaseElement>>,
}

pub fn run(mut args: Args, mut extensions: Vec<Box<dyn Extension>>) {
    for elem in &mut args.base_elements {
        elem.init();
    }

    loop {
        for extension in &mut extensions {
            extension.tick(&mut args);
        }

        thread::sleep(Duration::from_millis(1000 / args.tick_rate as u64));
    }
}
