use osui::prelude::*;

fn main() -> std::io::Result<()> {
    let screen = Screen::new();
    screen.extension(VelocityExtension);

    let count = use_state(0);

    rsx! {
        @Velocity(20, 0);
        @Transform::new();
        static "Count: {count}"
    }
    .draw(&screen);

    std::thread::spawn(move || loop {
        **count.get() += 1;
        std::thread::sleep(std::time::Duration::from_millis(100));
    });

    screen.run()
}
