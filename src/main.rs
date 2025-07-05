use osui::{
    elements::{
        div::Div,
        grid::{HGrid, VGrid},
    },
    style::Transform,
    Screen,
};

fn main() {
    let mut screen = Screen::new();

    let container = Div::new(0xffff00);

    screen
        .draw(container.clone())
        .component(Transform::center().dimensions(70, 11));

    let vgrid = VGrid::new(0x0000ff, 1);

    container.draw(screen.draw(vgrid.clone()).component(Transform::center()));

    for _ in 1..6 {
        let row = HGrid::new(0xff00ff, 1);

        vgrid.draw(screen.draw(row.clone()));

        for i in 1..10 {
            row.draw(
                screen
                    .draw((format!("Item {i}"), 0xff0000))
                    .component(Transform::new()),
            );
        }
    }

    screen.run().unwrap();
}
