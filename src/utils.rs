/// The `utils` module contains helper functions and utilities that assist in rendering and
/// managing UI state. These functions are used by UI elements like `Div` and `Text` for common
/// tasks such as frame creation, event handling, and component selection.
///
/// # Example
/// ```rust
/// use crate::ui::utils::{create_frame, render_to_frame};
///
/// let frame = create_frame(50, 10);
/// render_to_frame(crate::State::Normal, 50, &mut frame, &text_element);
/// ```
use crate::{Element, Value};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashMap,
    io::{stdout, Write},
};

lazy_static! {
    pub static ref ANSI: Regex = Regex::new(r"(\x1b\[([0-9;]*)[a-zA-Z])+").unwrap();
}

/// Compress a string by a regex pattern
pub fn compress_string(input: &str, re: &Regex) -> (String, HashMap<usize, String>) {
    let mut matches_map = HashMap::new();
    let mut res = String::new();
    let input_chars: Vec<char> = input.chars().collect();

    let mut i = 0;
    while i < input_chars.len() {
        if let Some(loc) = re.find(&input[i..]) {
            if loc.start() == 0 {
                let ansi_seq = &input[i..i + loc.end()];
                matches_map.insert(res.len(), ansi_seq.to_string());
                i += ansi_seq.chars().count();
            } else {
                res.push(input_chars[i]);
                i += 1;
            }
        } else {
            res.push(input_chars[i]);
            i += 1;
        }
    }

    (res, matches_map)
}

/// Merges a frame withe a line by x
fn merge_line(frame_: &str, line_: &str, x: usize) -> (String, String) {
    let (frame_, fm) = compress_string(frame_, &ANSI);
    let (mut line_, lm) = compress_string(line_, &ANSI);

    if let Some(_) = lm.get(&line_.len()) {
        line_.push('\n');
    }

    let mut res = String::new();
    let frame: Vec<char> = frame_.chars().collect();
    let line: Vec<char> = (line_).chars().collect();

    let flen = frame.len();
    let llen = line.len();

    for i in 0..flen {
        if let Some(v) = fm.get(&i) {
            res.push_str(v);
        }
        if i >= x && i - x < llen && line[i - x] != '\t' {
            if let Some(v) = lm.get(&(i - x)) {
                res.push_str(v);
            }
            if line[i - x] == '\n' {
                res.push(frame[i]);
            } else {
                res.push(line[i - x]);
            }
        } else {
            res.push(frame[i]);
        }
    }

    (res, line_)
}

pub struct Frame {
    frame: Vec<String>,
    width: usize,
    used: Vec<usize>,
}

impl Frame {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            frame: vec![" ".repeat(width); height],
            width,
            used: vec![0; height],
        }
    }
    pub fn render(&mut self, focused: bool, element: &Element) {
        // let data = element.get_data();
        let output = element.render(focused);
        let x;
        let mut y = 0;
        match output.1.y {
            crate::ui::Number::Px(_y) => {
                y = _y;
            }
            crate::ui::Number::Pe(p) => y = (self.frame.len() * p) / 100,
            crate::ui::Number::Center => {
                y = (self.frame.len() - output.0.matches('\n').count() + 1) / 2
            }
            crate::ui::Number::Auto => {
                for (i, n) in self.used.iter().enumerate() {
                    if *n == 0 {
                        y = i;
                        break;
                    }
                }
            }
            crate::ui::Number::Default => {
                for (i, n) in self.used.iter().enumerate() {
                    if *n == 0 {
                        y = i;
                        break;
                    }
                }
            }
        }
        match output.1.x {
            crate::ui::Number::Px(_x) => {
                x = _x;
            }
            crate::ui::Number::Pe(p) => x = (self.width * p) / 100,
            crate::ui::Number::Center => {
                x = (self.width
                    - ANSI
                        .replace_all(&output.0.lines().next().unwrap_or(""), "")
                        .len())
                    / 2;
            }
            crate::ui::Number::Auto => {
                x = *self.used.get(y).unwrap();
            }
            crate::ui::Number::Default => {
                x = 0;
            }
        }
        for (i, line) in output.0.split('\n').enumerate() {
            if self.frame.len() > y + i {
                let frame_line = self.frame.get_mut(y + i).unwrap();
                let merged = merge_line(&frame_line, line, x);
                *frame_line = merged.0;
                *self.used.get_mut(y + i).unwrap() += merged.1.len() - 1;
            }
        }
    }
    pub fn output(&self) -> String {
        self.frame.join("\n")
    }
    pub fn output_nnl(&self) -> String {
        self.frame.join("")
    }
}

pub fn clear() {
    print!("\x1B[2J\x1B[H");
    stdout().flush().unwrap();
}

pub fn hide_cursor() {
    print!("\x1b[?25l");
    stdout().flush().unwrap();
}

pub fn show_cursor() {
    print!("\x1B[?25h");
    stdout().flush().unwrap();
}

pub fn flush() {
    stdout().flush().unwrap();
}

#[derive(Debug, Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub fn closest_component(
    components: &[Element],
    current_index: usize,
    direction: Direction,
) -> usize {
    let current = &components[current_index].get_data();

    components
        .iter()
        .enumerate()
        .filter(|(i, comp_)| {
            let comp = comp_.get_data();
            match direction {
                Direction::Left => match comp.0 {
                    Value::Custom(_) => {
                        comp.0.get_value() < current.0.get_value() && comp.1 == current.1
                    }
                    Value::Default(_) => *i < current_index,
                }, // Left
                Direction::Right => match comp.0 {
                    Value::Custom(_) => {
                        comp.0.get_value() > current.0.get_value() && comp.1 == current.1
                    }
                    Value::Default(_) => *i > current_index,
                }, // Right
                Direction::Up => comp.1 < current.1 && comp.0.get_value() == current.0.get_value(), // Up
                Direction::Down => {
                    comp.1 > current.1 && comp.0.get_value() == current.0.get_value()
                } // Down
            }
        })
        .min_by_key(|(_, comp_)| {
            let comp = comp_.get_data();
            current.0.get_value().abs_diff(comp.0.get_value()) + current.1.abs_diff(comp.1)
        }) // Find the closest component
        .map(|(index, _)| index) // Return the index of the closest component
        .unwrap_or(current_index) // If no component is found, return the current index
}

pub fn get_term_size() -> (usize, usize) {
    let (width, height) = crossterm::terminal::size().unwrap();
    (width as usize, height as usize)
}
