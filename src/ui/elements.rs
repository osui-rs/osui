use std::collections::HashMap;

use crate::{
    command, element, key::Key, render_to_frame, ui::Style, Command, Direction, Element,
    ElementData, ElementSize, UpdateResponse,
};

element! {
    /// A text element for displaying static text in the TUI.
    ///
    /// The `Text` element displays text and does not respond to user interactions.
    Text {}
    defaults {}
    fn render(&self, _: usize) -> String {
        self.text.clone()
    }
}
element! {
    /// A clickable button element.
    ///
    /// The `Button` element can be clicked, triggering an `on_click` function. Its appearance changes
    /// based on its interaction state, such as being "clicked".
    Button {
        /// A callback function executed when the button is clicked.
        on_click: fn()
    }

    defaults {
        on_click: || {}
    }

    fn render(&self, state: usize) -> String {
        let writer = self.style.use_style(&state);
        if state == 2 {
            return writer.write_clicked(&self.text);
        }
        writer.write(&self.text)
    }

    fn event(&mut self, _state: usize, k: Key) -> UpdateResponse {
        if k == Key::Enter {
            (self.on_click)();
            return command!(
                Command::Render(2),
                Command::Sleep(120)
            );
        }
        UpdateResponse::None
    }
}

element! {
    /// A container element that can hold multiple child elements and handle directional key input.
    ///
    /// The `Div` element serves as a container for other elements, allowing navigation between them
    /// using directional keys.
    Div {
        pub keybinds: HashMap<Key, Direction>
    }

    defaults {
        keybinds: HashMap::from([
            (Key::Up, Direction::Up),
            (Key::Down, Direction::Down),
            (Key::Left, Direction::Left),
            (Key::Right, Direction::Right),
        ])
    }

    fn render(&self, state: usize) -> String {
        let mut frame = crate::create_frame(self.width, self.height);
        for (i, child) in (&self.children).iter().enumerate() {
            if i==self.child {
                render_to_frame(state, &mut frame, child);
            } else {
                render_to_frame(0, &mut frame, child);
            }
        }
        frame.join("\n")
    }

    fn event(&mut self, state: usize, k: Key) -> UpdateResponse {
        if let Some(direction) = self.keybinds.get(&k) {
            self.child = crate::closest_component(&self.children, self.child, direction.clone());
        } else if let Some(child) = self.get_child() {
            return child.event(state, k);
        }
        UpdateResponse::None
    }
}
