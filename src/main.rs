use osui::{
    extensions::{
        tick::{OnTick, TickExtension},
        velocity::{Velocity, VelocityExtension},
    },
    style::{Position, Transform},
    Screen,
};

fn main() {
    let mut screen = Screen::new();
    screen.extension(VelocityExtension);
    screen.extension(TickExtension(60));

    screen
        .draw(format!("Hello, World!"))
        .component(Transform::new())
        .component(Velocity(1, 0))
        .component(OnTick(|w| {
            if let Some(mut v) = w.get::<Velocity>() {
                if let Some(t) = w.get::<Transform>() {
                    if let Position::Const(c) = t.x {
                        if c >= 30 {
                            v.0 = 0;
                            w.set_component(v);
                        }
                    }
                }
            }
        }));

    screen.run().unwrap();
}
