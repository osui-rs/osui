use std::fmt::Debug;

use crate::event;
use crate::extensions::{Extension, ExtensionContext};

pub struct TickExtension(pub u16);

event!(TickEvent(pub u32));

impl Extension for TickExtension {
    fn init(&mut self, ctx: &ExtensionContext) {
        let ctx = ctx.clone();
        let rate_dur = 1000 / self.0 as u64;
        std::thread::spawn({
            move || {
                let mut tick = 0;
                loop {
                    ctx.event(&TickEvent(tick));
                    tick += 1;
                    std::thread::sleep(std::time::Duration::from_millis(rate_dur));
                }
            }
        });
    }
}
