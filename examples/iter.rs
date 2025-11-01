use osui::prelude::*;

fn main() -> std::io::Result<()> {
    let screen = Screen::new();

    app().draw(&screen);

    screen.run()
}

pub fn app() -> Rsx {
    let items: State<Vec<i32>> = use_state(Vec::new());
    let count = use_state(1);

    run! {
        ref count {
            loop {
                if count.get_dl() == 5 {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(300));
                **count.get() += 1;
            }
        }
    }

    run! {
        use count ref items {
            items.get().push(count.get_dl());
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
