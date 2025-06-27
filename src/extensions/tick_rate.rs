use crate::extensions::Extension;

pub struct TickRate(pub u8);

impl Extension for TickRate {
    fn init(&mut self, _: &mut crate::Screen, _: &mut crate::events::EventManager) {}

    fn tick_end(&mut self, _: &mut crate::Screen, _: &mut crate::events::EventManager) {
        std::thread::sleep(std::time::Duration::from_millis(1000 / self.0 as u64));
    }

    fn tick_start(&mut self, _: &mut crate::Screen, _: &mut crate::events::EventManager) {}
}
