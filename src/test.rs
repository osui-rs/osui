use crate::{rsx_elem, Element, ui::*};

pub(crate) fn app() -> Box<dyn Element> {
    rsx_elem! { text { "Hello, World!" } }
}