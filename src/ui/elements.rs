use crate::{element, State};

element! {
    Text {}
    defaults {}
    fn render(&self, _state: State) -> String {
        self.text.clone()
    }
}