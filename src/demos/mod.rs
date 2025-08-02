use osui::prelude::*;

pub fn app() -> Rsx {
    let count = use_state(0);

    std::thread::spawn({
        let count = count.clone();
        move || loop {
            **count.get() += 1;
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    rsx! {
        @Handler::new(|_, _: &crossterm::event::Event| {
            panic!("idk lol");
        });
        FlexRow {
            FlexCol {
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
            } (1)

            FlexCol {
                Div {
                    %count
                    "This will increment every second: {count}"
                }
            } (2)
        } (0)
    }
}
