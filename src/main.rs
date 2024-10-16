use osui::{
    components::{div, text},
    oml, Component, Params, Screen,
};

fn main() {
    let mut screen = Screen::new();
    screen.set_component(oml!(app()));
    screen.run();
}

fn app(_: Params) -> Component {
    oml!(
        // Components (C)
        div (width = 5, height = 3) {

            text("Hello, World! daddy"; width = 30)

            // // Props, With components (PC)
            // div () {
            //     // Props (P)
            //     text(/* Properties go here, Example: x = 20 y = 5 */)
            // }

            // // Expression (E)
            // text("Hello";)

            // // Expression, With components (EC)
            // text("World!";) {
            //     // Components in here
            // }
        }
    )
}
