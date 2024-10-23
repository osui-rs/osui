use crate::Component;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashMap,
    io::{stdout, Write},
};
lazy_static! {
    pub static ref ANSI: Regex = Regex::new(r"(\x1b\[([0-9;]*)[a-zA-Z])+").unwrap();
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    String(String),
    Int(i32),
    Float(f64),
    Bool(bool),
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
fn merge_line(frame_: &str, line_: &str, x: usize) -> String {
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
            if let Some(v) = fm.get(&i) {
                res.push_str(v);
            }
            res.push(frame[i]);
        }
    }

    res
}

/// Render to a frame
pub fn render_to_frame(frame: &mut Vec<String>, component: &mut Component) {
    for (i, line) in (component.render)(component).split('\n').enumerate() {
        if (component.y + i) < frame.len() {
            let frame_line = frame.get_mut(component.y + i).unwrap();
            *frame_line = merge_line(&frame_line, line, component.x);
        }
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
    print!("\x1b[?25H");
    stdout().flush().unwrap();
}

pub fn flush() {
    stdout().flush().unwrap();
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub fn closest_component(
    components: &[Component],
    current_index: usize,
    direction: Direction,
) -> usize {
    let current = &components[current_index];

    components
        .iter()
        .enumerate() // Keep track of indices
        .filter(|(_, comp)| match direction {
            Direction::Left => comp.x < current.x && comp.y == current.y, // Left
            Direction::Right => comp.x > current.x && comp.y == current.y, // Right
            Direction::Up => comp.y < current.y && comp.x == current.x,   // Up
            Direction::Down => comp.y > current.y && comp.x == current.x, // Down
        })
        .min_by_key(|(_, comp)| current.x.abs_diff(comp.x) + current.y.abs_diff(comp.y)) // Find the closest component
        .map(|(index, _)| index) // Return the index of the closest component
        .unwrap_or(current_index) // If no component is found, return the current index
}
