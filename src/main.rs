use osui::{
    extensions::{
        id::{Id, IdExtension},
        tick::{OnTick, TickExtension},
        Handler,
    },
    Screen,
};

fn main() {
    let mut screen = Screen::new();
    let document = IdExtension::new();
    screen.extension(document.clone());
    screen.extension(TickExtension(10));

    screen
        .draw(format!("Hello, World!"))
        .component(OnTick(Handler::new(move |_| {})))
        .component(Id(69));

    screen.run().unwrap();
}
