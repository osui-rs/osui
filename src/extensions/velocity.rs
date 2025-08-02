use std::sync::Arc;

use crate::{
    component,
    extensions::Extension,
    style::{Position, Transform},
    widget::Widget,
    Screen,
};

pub struct VelocityExtension;

component!(Velocity(pub i32, pub i32));

impl VelocityExtension {
    fn apply_velocity(ticks: u16, velocity: i32, x: &mut u16) {
        if velocity.abs() != 0 && ticks as i32 % (1000 / velocity.abs()) == 0 {
            if velocity > 0 {
                *x += 1;
            } else if velocity < 0 {
                *x -= 1;
            }
        }
    }

    fn apply_velocity_xy(ticks: u16, widget: &Arc<Widget>) {
        if let Some(velocity) = widget.get::<Velocity>() {
            if let Some(mut t) = widget.get::<Transform>() {
                match &mut t.x {
                    Position::Const(x) => Self::apply_velocity(ticks, velocity.0, x),
                    _ => {}
                }

                match &mut t.y {
                    Position::Const(y) => Self::apply_velocity(ticks, velocity.1, y),
                    _ => {}
                }

                widget.set_component(t);
            }
        }
    }
}

impl Extension for VelocityExtension {
    fn init(&mut self, screen: Arc<Screen>) {
        std::thread::spawn({
            move || {
                let mut tick = 0;
                loop {
                    for widget in screen.widgets.lock().unwrap().iter() {
                        Self::apply_velocity_xy(tick, widget);
                    }
                    if tick > 1000 {
                        tick = 0;
                    } else {
                        tick += 1;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }
            }
        });
    }
}
