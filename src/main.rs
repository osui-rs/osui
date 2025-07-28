use osui::{
    elements::{div::Div, flex::FlexRow},
    frontend::Rsx,
    rsx,
    state::use_state,
    style::Transform,
    Screen,
};

fn main() -> std::io::Result<()> {
    let screen = Screen::new();

    app().draw(&screen);

    screen.run()
}

fn app() -> Rsx {
    rsx! {
        FlexRow {
            counter()
        } (0x00000, 1)
    }
}

fn counter() -> Rsx {
    let count = use_state(0);

    std::thread::spawn({
        let count = count.clone();
        move || loop {
            std::thread::sleep(std::time::Duration::from_millis(50));
            **count.get() += 1;
        }
    });

    rsx! {
        @Transform::new().dimensions(40, 3);
        Div {
            %count
            "{count}"

            %count
            @Transform::center();
            "{count}"
        } (0xff0000)
    }
}
