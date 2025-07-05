use osui::{
    elements::grid::VGrid,
    extensions::{id::IdExtension, tick::TickExtension},
    style::Transform,
    Screen,
};

fn main() {
    let mut screen = Screen::new();
    let document = IdExtension::new();
    screen.extension(document.clone());
    screen.extension(TickExtension(10));

    let my_div = VGrid::new(0xff0000, 1);

    screen
        .draw(my_div.clone())
        .component(Transform::center().dimensions(16, 9));

    for i in 1..6 {
        my_div.draw(
            screen
                .draw(format!("Item {i}"))
                .component(Transform::center()),
        );
    }

    screen.run().unwrap();
}
