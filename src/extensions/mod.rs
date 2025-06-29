pub mod tick;
pub mod velocity;

use std::sync::Arc;

use crate::widget::Widget;

pub trait Extension {
    fn init(&self, _widgets: &Vec<Arc<Widget>>) {}
    fn render(&self, _widget: &Arc<Widget>) {}
}
