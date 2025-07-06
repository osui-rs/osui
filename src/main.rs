use osui::{
    elements::{
        div::Div,
        grid::{HGrid, VGrid},
    },
    extensions::{
        resources::Resource,
        tick::{TickEvent, TickExtension},
        Handler,
    },
    style::Transform,
    Screen,
};

fn main() {
    let mut screen = Screen::new();
    screen.extension(TickExtension(10));

    let container = Div::new(0xffff00);

    screen
        .draw(container.clone())
        .component(Transform::center().dimensions(70, 11));

    let vgrid = VGrid::new(0x0000ff, 1);

    container.draw(screen.draw(vgrid.clone()).component(Transform::center()));

    for _ in 1..6 {
        let row = HGrid::new(0xff00ff, 1);

        let row_widget = vgrid.draw(
            screen
                .draw(row.clone())
                .component(Resource(Vec::<&str>::from(["Test"])))
                .component(Handler::new(|w, t: &TickEvent| {
                    if t.0 >= 10 {
                        let mut res = w.get::<Resource<Vec<&str>>>().unwrap();
                        res.0.push("Hello World");
                        w.set_component(res);
                    }
                })),
        );

        for i in row_widget.get::<Resource<Vec<&str>>>().unwrap().0 {
            row.draw(
                screen
                    .draw((i.to_string(), 0xff0000))
                    .component(Transform::new()),
            );
        }
    }

    screen.run().unwrap();
}
