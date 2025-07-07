use std::{fmt::Debug, sync::Arc};

use crate::extensions::Extension;
use crate::extensions::Handler;
use crate::{event, Screen};

pub struct TickExtension(pub u16);

impl Extension for TickExtension {
    fn init(&mut self, screen: Arc<Screen>) {
        let rate_dur = 1000 / self.0 as u64;
        std::thread::spawn({
            move || {
                let mut tick = 0;
                loop {
                    for w in screen.widgets.lock().unwrap().iter() {
                        if let Some(on_tick) = w.get::<Handler<TickEvent>>() {
                            on_tick.call(&w, &TickEvent(tick))
                        }
                    }
                    tick += 1;
                    std::thread::sleep(std::time::Duration::from_millis(rate_dur));
                }
            }
        });
    }
}

event!(TickEvent(pub u32));
