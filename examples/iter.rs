use osui::prelude::*;

fn main() -> std::io::Result<()> {
    let screen = Screen::new();
    screen.extension(InputExtension);

    app().draw(&screen);

    screen.run()
}

pub fn app() -> Rsx {
    let items: State<Vec<i32>> = use_state(vec![1, 2]);
    let count = use_state(3);

    run! {
        use count ref items {
            items.get().push(count.get_dl());
            **count.get() += 1;
        }
    }

    rsx! {
        @Transform::new().dimensions(40, 3);
        Div {
            FlexRow, scroll: true, {
                for item in items {
                    static "{item}"
                }
            }
        }
    }
}
