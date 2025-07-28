use osui::{elements::grid::HGrid, rsx, state::use_state, style::Transform, Screen};

fn main() -> std::io::Result<()> {
    let screen = Screen::new();
    let count = use_state(0);

    rsx! {
        @Transform::new().dimensions(10, 1);
        HGrid {
            %count
            "{count}"

            %count
            @Transform::center();
            "{count}"
        } (0xff0000, 1)
    }
    .draw(&screen);

    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_millis(50));
        **count.get() += 1;
    });

    screen.run()
}
