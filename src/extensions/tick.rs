use std::sync::Arc;

use crate::{component, extensions::Extension, widget::Widget};

pub struct TickExtension(pub u16);

impl Extension for TickExtension {
    fn init(&self, widgets: &Vec<Arc<Widget>>) {
        let rate_dur = 1000 / self.0 as u64;
        std::thread::spawn({
            let widgets = widgets.clone();
            move || loop {
                for w in &widgets {
                    if let Some(on_tick) = w.get::<OnTick>() {
                        (on_tick.0)(&w)
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(rate_dur));
            }
        });
    }
}

component!(OnTick(pub fn(&Arc<Widget>)));
