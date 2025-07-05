use osui::{
    extensions::{
        id::{Id, IdExtension},
        tick::{TickEvent, TickExtension},
        Handler,
    },
    style::Transform,
    Screen,
};

fn main() {
    let mut screen = Screen::new();
    let document = IdExtension::new();
    screen.extension(document.clone());
    screen.extension(TickExtension(10));

    screen
        .draw(format!("Hello, World!"))
        .component(Handler::new(move |w, e: &TickEvent| {
            if e.0 == 30 {
                w.set_component(Transform::center());
            }
        }))
        .component(Id(69));

    screen.run().unwrap();
}
