use std::sync::Arc;

use crate::{
    component,
    extensions::Extension,
    style::{Position, Transform},
    widget::Widget,
};

pub struct VelocityExtension;

impl Extension for VelocityExtension {
    fn render(&self, _widget: &Arc<Widget>) {
        if let Some(velocity) = _widget.get::<Velocity>() {
            if let Some(mut t) = _widget.get::<Transform>() {
                match &mut t.x {
                    Position::Const(x) => {
                        if velocity.0 > 0 {
                            *x += 1;
                        } else if velocity.0 < 0 {
                            *x -= 1;
                        }
                    }
                    _ => {}
                }

                match &mut t.y {
                    Position::Const(y) => {
                        if velocity.1 > 0 {
                            *y += 1;
                        } else if velocity.1 < 0 {
                            *y -= 1;
                        }
                    }
                    _ => {}
                }

                _widget.set_component(t);
            }
        }
    }
}

component!(Velocity(pub i32, pub i32));
