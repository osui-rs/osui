use osui::{dependency::Dependency, elements::state::use_state, style::Transform, Screen};

fn main() -> std::io::Result<()> {
    let screen = Screen::new();
    let count = use_state(0);

    screen
        .draw(
            Dependency::new({
                let count = count.clone();
                move || format!("{count}")
            })
            .add(count.clone()),
        )
        .component(Transform::center());

    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_millis(900));
        let c = (*count).clone();
        count.set(c + 1);
    });

    screen.run()
}
