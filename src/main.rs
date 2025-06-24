use osui::{
    output::{OutputExtension, RenderAble},
    run, Args, BaseElement,
};

pub struct MyElem(*mut Vec<Box<dyn osui::ElementComponent>>);
impl BaseElement for MyElem {
    fn init(&mut self) {
        unsafe {
            (*self.0).push(RenderAble::new(|| String::from("value")));
        }
    }

    fn get_components<'a>(&'a mut self) -> &'a mut Vec<Box<dyn osui::ElementComponent>> {
        unsafe { &mut *self.0 }
    }
}

impl MyElem {
    pub fn new(mut v: Vec<Box<dyn osui::ElementComponent>>) -> Box<MyElem> {
        Box::new(MyElem(&mut v))
    }
}

fn main() {
    let args = Args {
        tick_rate: 40,
        base_elements: vec![],
    };
    run(args, vec![Box::new(OutputExtension)]);
}
