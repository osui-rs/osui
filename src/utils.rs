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
        let remaining_input: String = input_chars[i..].iter().collect();
        if let Some(loc) = re.find(&remaining_input) {
            if loc.start() == 0 {
                let matched_chars: String = input_chars[i..i + loc.end()].iter().collect();
                matches_map.insert(res.len(), matched_chars.clone());
                i += loc.end();
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
fn merge_line(
    frame_: &str,
    (mut line_, lm): (String, HashMap<usize, String>),
    x: usize,
) -> (String, String) {
    let (frame_, fm) = compress_string(frame_, &ANSI);

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
    pub fn render(&mut self, focused: bool, element: &crate::Element) {
        let output = element.render(focused, self.width, self.frame.len());
        if output.1.visible {
            let x;
            let mut y = 0;
            let mut width = 0_usize;
            match output.1.y {
                crate::ui::Number::Px(_y) => {
                    y = _y;
                }
                crate::ui::Number::Pe(p) => y = (self.frame.len() * p) / 100,
                crate::ui::Number::Center => {
                    let content_height = output.0.lines().count();
                    y = (self.frame.len() - content_height) / 2;
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

            let mut svec: Vec<(String, HashMap<usize, String>)> = Vec::new();

            match output.1.width {
                crate::ui::Number::Auto | crate::ui::Number::Default => {
                    for line in output.0.split('\n') {
                        let (line, lm) = compress_string(line, &ANSI);
                        if line.len() > width {
                            width = line.len();
                        }
                        svec.push((line, lm));
                    }
                }
                crate::ui::Number::Px(p) => {
                    width = p;
                    for line in output.0.split('\n') {
                        svec.push(compress_string(line, &ANSI));
                    }
                }
                _ => {}
            }

            if output.1.outline {
                let frame_line = self.frame.get_mut(y).unwrap();
                *frame_line = merge_line(&frame_line, ("_".repeat(width), HashMap::new()), x + 1).0;
                y += 1;
            }

            for (i, (line, lm)) in svec.iter().enumerate() {
                if self.frame.len() > y + i {
                    let frame_line = self.frame.get_mut(y + i).unwrap();
                    let merged = if output.1.outline {
                        merge_line(
                            &frame_line,
                            (
                                format!("|{}|", format_string(line, width)),
                                lm.into_iter().map(|(k, v)| (k + 1, v.clone())).collect(),
                            ),
                            x,
                        )
                    } else {
                        merge_line(&frame_line, (format_string(line, width), lm.clone()), x)
                    };
                    *frame_line = merged.0;
                    *self.used.get_mut(y + i).unwrap() += merged.1.len() - 1;
                }
            }

            if output.1.outline {
                let frame_line = self.frame.get_mut(y + svec.len()).unwrap();
                *frame_line = merge_line(&frame_line, ("¯".repeat(width), HashMap::new()), x + 1).0;
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

fn format_string(input: &str, width: usize) -> String {
    let truncated = if input.len() > width {
        &input[..width]
    } else {
        input
    };
    format!("{:<width$}", truncated, width = width)
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

pub fn get_term_size() -> (usize, usize) {
    let (width, height) = crossterm::terminal::size().unwrap();
    (width as usize, height as usize)
}
