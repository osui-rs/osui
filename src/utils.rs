use std::io::{stdout, Write};

use crate::Component;

pub fn render_to_frame(width: usize, frame: &mut Vec<String>, component: &mut Component) {
    for (i, line) in (component.render)(component).split('\n').enumerate() {
        if (component.y + i) < frame.len() {
            let frame_line = frame.get_mut(component.y + i).unwrap();
            // *frame_line = (&line[..line.len().saturating_sub(width)]).to_string();
            *frame_line = line.to_string();
        }
    }
}

pub fn clear() {
    print!("\x1b[2J\x1b[0;0H");
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
