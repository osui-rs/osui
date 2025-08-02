use std::sync::Arc;

use osui::prelude::*;

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
                    if *code == crossterm::event::KeyCode::Char('q') {
                        screen.close();
                    }
                }
            }});
        FlexRow {
            FlexCol, gap: 3, {
                Heading, smooth: false, { "OSUI" }
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

            FlexCol, gap: 2, {
                @transform!{ y: Center };
                static Div { // static only affects the element, not children
                    %count
                    "This will increment every second: {count}"
                }

                @Transform::new().padding(1, 1).dimensions(40, 1);
                @Style { foreground: Some(0xffffff), background: Background::RoundedOutline(0xff0000) };
                Input { }
            }
        }
    }
}
