use osui::prelude::*;

fn main() -> std::io::Result<()> {
    let screen = Screen::new();

    app().draw(&screen);

    screen.run()
}

pub fn app() -> Rsx {
    let items: State<Vec<i32>> = use_state(vec![1, 2]);

    std::thread::spawn({
        let items = items.clone();
        move || {
            std::thread::sleep(std::time::Duration::from_millis(1000));
            items.get().push(3);
            std::thread::sleep(std::time::Duration::from_millis(1000));
            items.get().remove(0);
        }
    });

    rsx! {
        FlexRow {
            for item in items {
                static "{item}"
                static "Idk {item}"
            }
        }
    }
}
