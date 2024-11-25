/// The `ui` module defines the basic building blocks for creating a terminal-based UI framework.
///
/// # Modules
/// - `styles`: A module for defining UI styles.
pub mod styles;
pub use styles::*;

use crate::{prelude::*, RenderResult, RenderWriter};
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
pub struct Text {
    pub on_event: Handler<Text<'a>>,
}

impl ElementWidget for Text<'_> {
    fn render(&self, focused: bool) -> RenderResult {
        let mut writer = RenderWriter::new(focused, self.style.clone());

        writer.write(&{
            if let Children::Text(text) = &self.children {
                text.clone()
            } else {
                String::new()
            }
        });

        writer.result()
    }
    fn event(&mut self, event: Event, document: &Document) {
        call!(self.on_event(event, document));
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
pub struct Div {
    pub styling: Option<std::collections::HashMap<StyleName, Style>>,
}

impl ElementWidget for Div<'_> {
    fn render(&self, focused: bool) -> RenderResult {
        let mut writer = RenderWriter::new(focused, self.style.clone());
        let mut frame = crate::utils::Frame::new(crate::utils::get_term_size().0, 12);

        if let Children::Children(children, child) = &self.children {
            for (i, elem) in children.iter().enumerate() {
                frame.render(focused && i == *child, elem);
            }
        }
        writer.write(&frame.output());

        writer.result()
    }

    fn event(&mut self, event: Event, document: &Document) {
        if event == Event::FocusGained {
            if let Some(styling) = self.styling.clone() {
                self.set_styling(&styling);
            }
        }
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
                                c.event(event, document);
                            }
                            *child
                        }
                    }
                }
            }
            _ => {
                if let Some(child) = self.get_child() {
                    child.event(event, document);
                }
            }
        }
    }
}

#[element]
#[elem_fn]
#[derive(Default, Debug)]
pub struct Button {
    pub on_click: Handler<Button<'a>>,
}

impl ElementWidget for Button<'_> {
    fn render(&self, focused: bool) -> RenderResult {
        let mut writer = RenderWriter::new(focused, self.style.clone());
        writer.write(&self.children.get_text());
        writer.result()
    }

    fn event(&mut self, event: Event, document: &Document) {
        match event {
            Event::Key(key) => {
                if key.code == KeyCode::Enter {
                    document.render();
                    self.style.set_state("clicked");
                    call!(self.on_click(event, document));
                    document.render();
                    sleep(100);
                    self.style.set_state("");
                }
            }
            _ => {}
        }
    }
}
