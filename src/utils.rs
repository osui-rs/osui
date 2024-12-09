use std::io::{stdout, Write};

pub fn overlay(layer0: &str, layer1: &str, mut index: usize) -> String {
    let layer0: Vec<char> = layer0.chars().collect();
    let layer1: Vec<char> = layer1.chars().collect();
    let mut layer1_index = 0;
    let mut result = String::new();
    let mut is_ansi = false;
    let mut ansi = String::new();

    for (i, &c) in layer0.iter().enumerate() {
        if c == '\0' {
            if is_ansi {
                result.push_str(&ansi);
                ansi.clear();
            }
            is_ansi = !is_ansi;
            index += 1;
        } else if is_ansi {
            ansi.push(c);
            index += 1;
        } else if i >= index && layer1_index < layer1.len() {
            if layer1[layer1_index] == '\0' {
                result.push(c);
            }
            result.push(layer1[layer1_index]);
            layer1_index += 1;
        } else {
            result.push(c);
        }
    }

    result
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
        let style = element.get_style().get(focused);
        let mut writer = crate::Writer::new(
            focused,
            style.clone(),
            (self.width, self.frame.len() as u16),
        );
        if style.visible {
            let frame_height = self.frame.len() as u16;
            let (width, height) = writer.get_size();
            let y = style.y.as_position_y(&self.used, height, frame_height) as usize;
            if let Some(y_) = self.used.get(y) {
                let x = style.x.as_position_x(y_, width, self.width) as usize;
                if style.outline {
                    // {
                    //     let frame_line = self.frame.get_mut(y).unwrap();
                    //     *frame_line =
                    //         overlay(&frame_line, &"_".repeat(width as usize - 2), x as usize);
                    //     // *self.used.get_mut(y - 1).unwrap() += (x as u16 + width + 1) as u16;
                    // }
                    // {
                    //     if let Some(frame_line) = self.frame.get_mut(y + height as usize) {
                    //         *frame_line = overlay(
                    //             &frame_line,
                    //             &"⎻".repeat(width as usize - 2),
                    //             x as usize + 1,
                    //         );
                    //         // *self.used.get_mut(y + height as usize).unwrap() +=
                    //         //     (x as u16 + width + 1) as u16;
                    //     }
                    // }
                    // y += 1;
                }
                element.render(&mut writer);
                for (i, line) in writer.text.lines().enumerate() {
                    if frame_height > (y + i) as u16 {
                        let frame_line = self.frame.get_mut(y + i).unwrap();
                        if style.outline {
                            *frame_line = overlay(frame_line, &format!("a{line}b"), x as usize);
                        } else {
                            *frame_line = overlay(frame_line, line, x as usize);
                        }
                        // *self.used.get_mut(y + i).unwrap() += (x + formatted.len()) as u16;
                    }
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

    pub fn output_array(&self) -> &Vec<String> {
        &self.frame
    }
}

// fn format_string(input: &str, width: usize, outline: bool) -> String {
//     let truncated = if input.len() > width {
//         let input_chars: Vec<char> = input.chars().collect();
//         input_chars[..width].iter().collect::<String>()
//     } else {
//         input.to_string()
//     };
//     if outline {
//         format!("a{:<width$}b", truncated, width = width)
//     } else {
//         format!("{:<width$}", truncated, width = width)
//     }
// }

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
