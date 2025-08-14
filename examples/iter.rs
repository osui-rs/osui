use osui::prelude::*;

fn main() -> std::io::Result<()> {
    let screen = Screen::new();

    app().draw(&screen);

    screen.run()
}

pub fn app() -> Rsx {
    let items: State<Vec<i32>> = use_state(vec![1, 2]);

    run! {
        use items
        {
            std::thread::sleep(std::time::Duration::from_millis(300));
            items.get().push(3);
        }
    }

    rsx! {
        FlexRow {
            for item in items {
                static "{item}"
            }
        }
    }
}
