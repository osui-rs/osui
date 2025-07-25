use osui::{elements::state::use_state, Screen};

fn main() -> std::io::Result<()> {
    let screen = Screen::new();
    let count = use_state(0);

    screen.draw(count.draw({
        let count = count.clone();
        move || Box::new(format!("Count: {count}"))
    }));

    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_millis(500));
        count.set(unsafe { *count.value } + 1);
    });

    screen.run()
}
