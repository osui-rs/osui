use std::sync::Arc;

use crate::{component, extensions::Extension, widget::Widget};

pub struct TickExtension;

impl Extension for TickExtension {
    fn render(&self, _widget: &Arc<Widget>) {
        if let Some(on_tick) = _widget.get::<OnTick>() {
            (on_tick.0)(_widget)
        }
    }
}

component!(OnTick(pub fn(&Arc<Widget>)));
