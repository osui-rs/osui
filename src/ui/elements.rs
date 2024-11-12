use std::sync::Mutex;

use crate::{element, Handler, State};

element! {
    Text {
        pub on_click: Mutex<Handler<Text<'a>>>
    }
    defaults {
        on_click: Handler::new(|_, _, _| {})
    }
    fn render(&self, _: State) -> String {
        self.text.lock().unwrap().clone()
    }

    fn event(&self, ch: &crate::CommandHandler, event: crossterm::event::Event) {
        (*self.on_click.lock().unwrap().0)(self, ch, event);
        // *self.text.lock().unwrap() = "Done!".to_string();
    }
}

element! {
    Div {}
    defaults {}
    fn render(&self, s: State) -> String {
        if let Some(child) = self.get_child() {
            child.render(s)
        } else {
            format!("None")
        }
    }

    fn event(&self, ch: &crate::CommandHandler, event: crossterm::event::Event) {
        if let Some(e) = self.get_element_by_id("test") {
            e.event(ch, event);
        }
    }
}
