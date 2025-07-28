use osui::{rsx, state::use_state, Screen};

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

    // let mut r = Rsx(Vec::new());
    // r.create_element(
    //     {
    //         let count = count.clone();
    //         move || WidgetLoad::new(format!("{count}"))
    //     },
    //     vec![Box::new(count)],
    // );

    // r.draw(&screen);

    rsx! {
        %count
        "{count}"
    }
    .draw(&screen);

    screen.run()
}
