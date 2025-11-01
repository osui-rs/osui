use osui::prelude::*;

fn main() -> std::io::Result<()> {
    let screen = Screen::new();

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
        // @transform! {
        //     height: 3
        // };
        // FlexRow {
        //     for item in items {
        //         static "{item}"
        //     }
        // }

        @transform! {
            height: 3
        };
        FlexRow {
            @transform! {
                height: 1
            };
            @Style { foreground: None, background: Background::Solid(0xff0000) };
            Div { "yo" }

            @transform! {
                height: 1
            };
            @Style { foreground: None, background: Background::Solid(0x00ff00) };
            Div { "gurt" }

            @transform! {
                height: 1
            };
            @Style { foreground: None, background: Background::Solid(0x0000ff) };
            Div { "67" }

            @transform! {
                height: 1
            };
            @Style { foreground: None, background: Background::Solid(0x0000ff) };
            Div { "41" }
        }
    }
}
