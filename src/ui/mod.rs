/// The `ui` module defines the basic building blocks for creating a terminal-based UI framework.
/// This includes elements like `Text` and `Div`, which can be combined to create complex layouts
/// and interactive UIs in the terminal. It also provides utility functions for rendering, event
/// handling, and managing child components.
///
/// Key Components:
/// - `Text`: A simple text element that renders a string to the terminal.
/// - `Div`: A container element that holds and renders child components, allowing navigation
///   through them with keyboard events (Up, Down, Left, Right).
/// - `styles`: A module for defining UI styles.
/// - `utils`: Utility functions for rendering and managing state.
pub mod styles;
pub use styles::*;

use crate::prelude::*;
use osui_element::{elem_fn, element};

/// The `Text` element represents a piece of static text in the terminal UI.
///
/// It is used to render plain text within a UI component. The `Text` element does not interact
/// with events and only outputs the text when rendered.
///
/// # Example
/// ```rust
/// let text = Text::default();
/// let rendered_text = text.render(crate::State::Normal);
/// println!("{}", rendered_text);
/// ```
///
/// ## Methods
/// - `render(state: crate::State) -> String`: Renders the text to a string, depending on the state.
///   If the element contains text as a child, it returns that text. Otherwise, it returns an empty string.
#[element]
#[elem_fn]
#[derive(Default, Debug)]
pub struct Text {}

impl Component for Text<'_> {
    fn render(&self, _: crate::State) -> String {
        if let Children::Text(text) = &self.children {
            text.clone()
        } else {
            String::new()
        }
    }
}

/// The `Div` element represents a container for other UI elements. It can hold child components
/// and render them in a specific layout. The `Div` element also handles keyboard navigation through
/// its children.
///
/// The `Div` element supports rendering a list of children and allows interactive navigation using
/// arrow keys (Up, Down, Left, Right). It renders the selected child element differently based on the state.
///
/// # Example
/// ```rust
/// let div = Div::default();
/// let rendered_div = div.render(crate::State::Normal);
/// println!("{}", rendered_div);
/// ```
///
/// ## Methods
/// - `render(state: crate::State) -> String`: Renders the `Div` and its child elements to a string.
///   The child elements are rendered according to the current selection (`child` index).
/// - `event(event: Event, state: &crate::StateChanger)`: Handles keyboard events for
///   navigation through child elements. Arrow keys (Up, Down, Left, Right) change the currently selected child.

#[element]
#[elem_fn]
#[derive(Default, Debug)]
pub struct Div {}

impl Component for Div<'_> {
    fn render(&self, state: State) -> String {
        let mut frame = crate::utils::create_frame(self.width.get_value(), self.height.get_value());

        if let Children::Children(children, child) = &self.children {
            for (i, elem) in children.iter().enumerate() {
                if i == *child {
                    crate::utils::render_to_frame(state, self.width.get_value(), &mut frame, elem);
                } else {
                    crate::utils::render_to_frame(
                        crate::State::Normal,
                        self.width.get_value(),
                        &mut frame,
                        elem,
                    );
                }
            }
        }

        frame.join("\n")
    }

    fn event(&mut self, event: Event, state: &crate::StateChanger) {
        match event {
            Event::Key(key) => {
                if let Children::Children(children, child) = &mut self.children {
                    *child = match key.code {
                        KeyCode::Up => crate::utils::closest_component(
                            children,
                            *child,
                            crate::utils::Direction::Up,
                        ),

                        KeyCode::Down => crate::utils::closest_component(
                            children,
                            *child,
                            crate::utils::Direction::Down,
                        ),

                        KeyCode::Left => crate::utils::closest_component(
                            children,
                            *child,
                            crate::utils::Direction::Left,
                        ),

                        KeyCode::Right => crate::utils::closest_component(
                            children,
                            *child,
                            crate::utils::Direction::Right,
                        ),

                        _ => {
                            if let Some(c) = children.get_mut(*child) {
                                c.event(event, state);
                            }
                            *child
                        }
                    }
                }
            }
            _ => {
                if let Some(child) = self.get_child() {
                    child.event(event, state);
                }
            }
        }
    }
}

#[element]
#[elem_fn]
#[derive(Default, Debug)]
pub struct Button {}

impl Component for Button<'_> {
    fn render(&self, state: State) -> String {
        match state {
            State::Custom(_) => format!("\x1b[32m{}\x1b[0m", self.children.get_text()),
            _ => self.children.get_text(),
        }
    }

    fn event(&mut self, event: Event, state: &StateChanger) {
        match event {
            Event::Key(key) => {
                if key.code == KeyCode::Enter {
                    let prev_state = state.get_state();
                    state.set_state(State::Custom(0));
                    sleep(100);
                    state.set_state(prev_state);
                }
            }
            _ => {}
        }
    }
}
