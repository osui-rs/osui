use osui::{rsx, state::use_state, style::Transform, Screen};

fn main() -> std::io::Result<()> {
    let screen = Screen::new();
    let count = use_state(0);

    rsx! {
        %count
        @Transform::center();
        "{count}"
    }
    .draw(&screen);

    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_millis(50));
        **count.get() += 1;
    });

    screen.run()
}
