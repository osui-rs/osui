use osui::{dependency::Dependency, elements::state::use_state, style::Transform, Screen};

fn main() -> std::io::Result<()> {
    let screen = Screen::new();
    let count = use_state(0);

    std::thread::spawn({
        let count = count.clone();
        move || loop {
            std::thread::sleep(std::time::Duration::from_millis(1000));
            **count.get() += 500;
        }
    });

    screen
        .draw(
            Dependency::new({
                let count = count.clone();
                move || format!("{count}")
            })
            .add(count.clone()),
        )
        .component(Transform::center());

    screen.run()
}
