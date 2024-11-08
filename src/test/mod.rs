use crate::{self as osui, rsx, ui::*, val, Element};

pub fn app() -> Box<dyn Element> {
    rsx! {
        custom_component { width: val!(20), "" }
    }
}

pub fn custom_component<'a>() -> Box<Div<'a>> {
    rsx! {
        button { style.clicked_background: Color::Red, "test" }
    }
}
