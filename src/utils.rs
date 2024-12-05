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
fn merge_line(frame_: &str, (mut line_, lm): (String, HashMap<usize, String>), x: usize) -> String {
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

    res
}

pub struct Frame {
    frame: Vec<String>,
    width: u16,
    used: Vec<u16>,
}

impl Frame {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            frame: vec![" ".repeat(width.into()); height.into()],
            width,
            used: vec![0; height.into()],
        }
    }
    pub fn render(&mut self, focused: bool, element: &crate::Element) {
        let mut writer = crate::Writer::new(
            focused,
            element.get_style().clone(),
            (self.width, self.frame.len() as u16),
        );
        let style = writer.style.get(focused);
        element.render(&mut writer);
        if style.visible {
            let frame_height = self.frame.len() as u16;
            let width = style.width.as_size(writer.text.len() as u16, self.width);
            let height = style.height.as_size(0, frame_height);
            let y = style.y.as_position_y(&self.used, height, frame_height) as usize;
            let x = style
                .x
                .as_position_x(self.used.get(y).unwrap(), width, self.width)
                as usize;

            for (i, line) in writer.text.lines().enumerate() {
                if frame_height > (y + i) as u16 {
                    let frame_line = self.frame.get_mut(y + i).unwrap();
                    let (line_, lm) = compress_string(line, &ANSI);
                    let formatted = format_string(&line_, width as usize);
                    *frame_line =
                        merge_line(&frame_line, (formatted.clone(), lm.clone()), x as usize);
                    *self.used.get_mut(y + i).unwrap() += (x + formatted.len()) as u16;
                }
            }
        }
    }
    /// Gets the frame output on a single string, rows separated by newlines
    pub fn output(&self) -> String {
        self.frame.join("\n")
    }

    /// Gets the frame output on a single string with no newlines
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
