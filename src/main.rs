use osui::{
    text::{text, DisplayComponent},
    Element,
};

fn main() {
    let mut e = text("Hello, World!");

    if let Some(comp) = e.get_component::<DisplayComponent>() {
        println!("Ok: {:?}", comp);
    }
}
