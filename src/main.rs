use osui::{elements::div::Div, rsx, state::use_state, style::Transform, Screen};

fn main() -> std::io::Result<()> {
    let screen = Screen::new();
    let count = use_state(0);

    rsx! {
        @Transform::new().dimensions(40, 3);
        Div {
            %count
            "{count}"

            %count
            @Transform::center();
            "{count}"
        } (0xff0000)
    }
    .draw(&screen);

    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_millis(50));
        **count.get() += 1;
    });

    screen.run()
}
