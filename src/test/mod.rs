use crate::{self as osui, rsx, ui::*, Element};

pub fn app() -> Box<dyn Element> {
    rsx! {
        button { "ok" }
    }
}
