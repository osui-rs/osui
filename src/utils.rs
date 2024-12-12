use std::io::{stdout, Write};

use crossterm::terminal::disable_raw_mode;

pub struct Frame {
    used: Vec<u16>,
    width: u16,
    height: u16,
    x: u16,
    y: u16,
}

impl Frame {
    pub fn new((x, y): (u16, u16), (width, height): (u16, u16)) -> Self {
        Frame {
            used: vec![0; height as usize],
            width,
            height,
            x,
            y,
        }
    }
    pub fn render(&mut self, focused: bool, element: &crate::Element) {
        let style = element.get_style().get(focused);
        if style.visible {
            let y = style.y.as_position(
                &{
                    let mut y_ = 0;
                    for n in self.used.iter().rev() {
                        if *n == 0 {
                            break;
                        }
                        y_ += 1;
                    }
                    y_
                },
                self.height,
            );
            if let Some(y_) = self.used.get(y as usize) {
                let x = style.x.as_position(y_, self.width);
                let mut writer = crate::Writer::new(
                    focused,
                    style.clone(),
                    (x + self.x, y + self.y),
                    (self.width, self.height),
                );
                element.render(&mut writer);
                let (width, height) = writer.get_size_root();
                if style.outline {
                    clear();
                    show_cursor();
                    disable_raw_mode().unwrap();
                    panic!("the 'outline' option isn't available right now. please set 'outline' to 'false'")
                }
                for i in y..y + height {
                    if let Some(o) = self.used.get_mut(i as usize) {
                        *o += x + width;
                    }
                }
            }
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
    print!("\x1B[?25h");
    stdout().flush().unwrap();
}

pub fn flush() {
    stdout().flush().unwrap();
}
