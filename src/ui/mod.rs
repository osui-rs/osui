pub mod styles;
pub use styles::*;
pub mod elements;
pub use elements::*;

pub fn text() -> Box<Text> {
    Box::new(Text::new())
}
