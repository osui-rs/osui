use crate::{element, State};

element! {
    Text {}
    defaults {}
    fn render(&self, _: State) -> String {
        self.text.clone()
    }
}
