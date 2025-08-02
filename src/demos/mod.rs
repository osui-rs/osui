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
        FlexRow {
            FlexCol {
                @Transform::new().padding(2, 2);
                @Style { foreground: None, background: Background::RoundedOutline(0x00ff00) }; // TODO: Outline doesn't work
                Div {
                    "This is text inside a div"
                }

                // TODO: div with width full
            } (2)

            FlexCol {
                Div {
                    %count
                    "This will increment every second: {count}"
                }
            } (2)
        } (0)
    }
}
