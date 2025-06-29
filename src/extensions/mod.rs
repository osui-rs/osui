pub mod tick;
pub mod velocity;

use std::sync::Arc;

use crate::widget::Widget;

pub trait Extension {
    fn render(&self, _widget: &Arc<Widget>) {}
}
