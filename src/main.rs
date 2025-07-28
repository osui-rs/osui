use osui::{state::use_state, widget::WidgetLoad, Screen};

fn main() -> std::io::Result<()> {
    let screen = Screen::new();
    let count = use_state(0);

    std::thread::spawn({
        let count = count.clone();
        move || loop {
            std::thread::sleep(std::time::Duration::from_millis(50));
            **count.get() += 1;
            count.update();
        }
    });

    screen
        .draw({
            let count = count.clone();
            move || WidgetLoad::new(format!("Hello, World {count}"))
        })
        .dependency(count);

    screen.run()
}
