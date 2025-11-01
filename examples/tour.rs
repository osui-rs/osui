use std::sync::Arc;

use osui::prelude::*;

fn main() -> std::io::Result<()> {
    let screen = Screen::new();
    screen.extension(InputExtension);
    screen.extension(RelativeFocusExtension::new());

    app(screen.clone()).draw(&screen);

    screen.run()
}

pub fn app(screen: Arc<Screen>) -> Rsx {
    let count = use_state(0);

    std::thread::spawn({
        let count = count.clone();
        move || loop {
            **count.get() += 1;
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    rsx! {
        @Handler::new({
            let screen = screen.clone();
            move |_, e: &crossterm::event::Event| {
                if let crossterm::event::Event::Key(crossterm::event::KeyEvent { code, .. }) = e {
                    if *code == crossterm::event::KeyCode::Esc {
                        screen.close();
                    }
                }
            }});
        @AlwaysFocused;
        Paginator {
            FlexRow {
                Heading, smooth: false, { "OSUI" }
                "Welcome to the OSUI demo!"
                "Press tab to switch to the next page or shift+tab to the previous page"
            }

            FlexCol, gap: 3, {
                @Transform::new().padding(2, 2);
                @Style { foreground: None, background: Background::RoundedOutline(0x00ff00) };
                Div {
                    "This is text inside a div"
                }

                @Transform::new().padding(2, 2);
                @Style { foreground: None, background: Background::Outline(0x00ff00) };
                Div {
                    "This is text inside a div with square outlines"
                }

                // TODO: div with width full
            }

            FlexRow, gap: 1, {
                %count
                "This will increment every second: {count}"

                FlexRow
                {
                    "Username"
                    @Transform::new().padding(1, 1).dimensions(40, 1);
                    @Style { foreground: Some(0xffffff), background: Background::RoundedOutline(0xff0000) };
                    @Focused;
                    Input { }
                }

                FlexRow
                {
                    "Password"
                    @Transform::new().padding(1, 1).dimensions(40, 1);
                    @Style { foreground: Some(0xffffff), background: Background::RoundedOutline(0xffff00) };
                    Input { }
                }
            }
        }
    }
}
