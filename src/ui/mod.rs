/// The `ui` module provides user interface components and utilities for building
/// a text-based user interface (TUI). It includes predefined styles and elements
/// such as text, buttons, and containers.
pub mod styles;
pub use styles::*;

use crate::{Children, Element, ElementComponent, Value};
use osui_element::{elem_fn, element};

#[element]
#[elem_fn]
#[derive(Default, Debug)]
pub struct Text {}

impl Element for Text<'_> {
    fn render(&self, _: crate::State) -> String {
        if let Children::Text(text) = &self.children {
            text.clone()
        } else {
            String::new()
        }
    }
}

#[element]
#[elem_fn]
#[derive(Default, Debug)]
pub struct Div {}

impl Element for Div<'_> {
    fn render(&self, state: crate::State) -> String {
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

    fn event(&mut self, event: crossterm::event::Event, state: &crate::StateChanger) {
        match event {
            crossterm::event::Event::Key(key) => {
                if let Children::Children(children, child) = &mut self.children {
                    *child = match key.code {
                        crossterm::event::KeyCode::Up => crate::utils::closest_component(
                            children,
                            *child,
                            crate::utils::Direction::Up,
                        ),

                        crossterm::event::KeyCode::Down => crate::utils::closest_component(
                            children,
                            *child,
                            crate::utils::Direction::Down,
                        ),

                        crossterm::event::KeyCode::Left => crate::utils::closest_component(
                            children,
                            *child,
                            crate::utils::Direction::Left,
                        ),

                        crossterm::event::KeyCode::Right => crate::utils::closest_component(
                            children,
                            *child,
                            crate::utils::Direction::Right,
                        ),

                        _ => *child,
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
