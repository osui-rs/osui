use osui::{
    elements::grid::VGrid,
    extensions::{
        resources::VecResource,
        tick::{TickEvent, TickExtension},
        Handler,
    },
    style::Transform,
    Screen,
};

fn main() {
    let screen = Screen::new();
    screen.extension(TickExtension(10));

    let items = <VecResource<&str>>::new();

    let vgrid = VGrid::new(0x000000, 0);

    screen
        .draw(vgrid.clone())
        .component(Transform::center())
        .component(Handler::new({
            let items = items.clone();
            move |_, e: &TickEvent| {
                if e.0 == 1 {
                    items.push("Hello?");
                }
            }
        }));

    items.iterate({
        let screen = screen.clone();
        move |i| {
            vgrid.draw(
                screen
                    .draw((i.to_string(), 0xff0000))
                    .component(Transform::new()),
            );
        }
    });

    screen.run().unwrap();
}
