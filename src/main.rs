use osui::{
    elements::grid::{HGrid, VGrid},
    extensions::{id::IdExtension, tick::TickExtension},
    style::Transform,
    Screen,
};

fn main() {
    let mut screen = Screen::new();
    let document = IdExtension::new();
    screen.extension(document.clone());
    screen.extension(TickExtension(10));

    let vgrid = VGrid::new(0xff0000, 1);

    screen
        .draw(vgrid.clone())
        .component(Transform::center().dimensions(34, 9));

    for _ in 1..6 {
        let row = HGrid::new(0x000000, 1);

        vgrid.draw(
            screen
                .draw(row.clone())
                .component(Transform::new().dimensions(5, 1)),
        );

        for i in 1..6 {
            row.draw(
                screen
                    .draw((format!("Item {i}"), 0xff0000))
                    .component(Transform::new()),
            );
        }
    }

    screen.run().unwrap();
}
