use crate::{element, ui::Style, Element, ElementData};

element! {
    Text {}
    defaults {}
    fn render(&mut self) -> String {
        self.style.write(&self.text)
    }
}
