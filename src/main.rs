use osui::{
    elements::{div::Div, flex::FlexCol},
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
        @Transform::center();
        FlexCol {
            test => (1)
            test => (3)
        } (0x0000ff, 4)
    }
}

fn test(inc: i32) -> Rsx {
    let count = use_state(0);

    std::thread::spawn({
        let count = count.clone();
        move || loop {
            std::thread::sleep(std::time::Duration::from_millis(50));
            **count.get() += inc;
        }
    });

    rsx! {
        @Transform::new().dimensions(40, 3);
        Div {
            @Transform::center().dimensions(8, 1);
            Div {
                %count
                "{count}"
            } (0x00ff00)
        } (0xff0000)
    }
}
