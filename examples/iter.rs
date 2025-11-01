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
        ref count {
            loop {
                if count.get_dl() == 20 {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(200));
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
