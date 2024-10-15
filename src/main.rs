use osui::{components::div, oml, Component, ComponentParams};

fn main() {
    let doc = oml!(
        (div) {
            (div) {}
        }
    );

    println!("{:?}", doc);
}
