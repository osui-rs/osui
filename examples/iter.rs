use std::sync::Arc;

use osui::prelude::*;

fn main() -> std::io::Result<()> {
    let screen = Screen::new();
    screen.extension(InputExtension);

    app(screen.clone()).draw(&screen);

    screen.run()
}

pub fn app(screen: Arc<Screen>) -> Rsx {
    let items: State<Vec<i32>> = use_state(Vec::new());
    let count = use_state(1);

    run! {
        ref count, items {
            loop {
                items.get().push(count.get_dl());
                **count.get() += 1;
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }

    rsx! {
        @AlwaysFocused;
        @Handler::new({
            let screen = screen.clone();
            move |_, e: &crossterm::event::Event| {
                if let crossterm::event::Event::Key(crossterm::event::KeyEvent { code, .. }) = e {
                    if *code == crossterm::event::KeyCode::Esc {
                        screen.close();
                    }
                }
            }
        });
        Scroll {
            FlexRow {
                for item in items {
                    static "{item}"
                }
            }
        }
    }
}
