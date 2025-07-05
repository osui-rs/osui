use osui::{
    elements::div::Div,
    extensions::{id::IdExtension, tick::TickExtension},
    style::Transform,
    Screen,
};

fn main() {
    let mut screen = Screen::new();
    let document = IdExtension::new();
    screen.extension(document.clone());
    screen.extension(TickExtension(10));

    let my_div = Div::new(0xff0000);

    screen
        .draw(my_div.clone())
        .component(Transform::center().dimensions(21, 3));

    my_div.draw(
        screen
            .draw(format!("Hello, World!"))
            .component(Transform::center()),
    );

    screen.run().unwrap();
}
