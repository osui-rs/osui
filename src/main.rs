use osui::{examples::todo_app, prelude::*};

fn main() {
    launch!(todo_app())
}

// use osui::utils::overlay;

// fn main() {
//     let layer0 = " ".repeat(crossterm::terminal::size().unwrap().0 as usize);
//     let layer1 = "\0\x1b[32m\0ABC";
//     let overlayed = overlay(&layer0, &layer1, 0).replace('\0', "");
//     println!("{overlayed}END");
// }
