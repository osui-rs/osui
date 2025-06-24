pub mod text;
pub mod macros;

use std::fmt::Debug;

use downcast_rs::{impl_downcast, DowncastSync};
pub trait Component: DowncastSync + Debug {}
impl_downcast!(sync Component);

pub trait Element {
    fn get_component<T: Component + 'static>(&mut self) -> Option<&mut T>;
    fn add_component<T: Component + 'static>(&mut self, component: T);
}

pub trait Extension {
    fn tick(&mut self);
}
