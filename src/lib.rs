use std::io::{self, Write};

pub mod ui;

pub struct Component<T> {
    pub component: T,
    pub x: usize,
    pub y: usize,
    cleared_frame: String,
    pub clear_trace: bool,
}

impl<T: std::fmt::Display> Component<T> {
    pub fn new(component: T) -> Component<T> {
        Component {
            component,
            x: 0,
            y: 0,
            cleared_frame: String::new(),
            clear_trace: false,
        }
    }
    pub fn render(&mut self) {
        if self.clear_trace {
            self.clear();
        }
        self.cleared_frame.clear();
        for (i, d) in format!("{}", self.component).split("\n").enumerate() {
            print!("\x1B[{};{}H{}", self.y + (i + 1), self.x, d);
            self.cleared_frame += format!(
                "\x1B[{};{}H{}",
                self.y + (i + 1),
                self.x,
                " ".repeat(d.len())
            )
            .as_str();
        }
        flush();
    }
    pub fn clear(&self) {
        print!("{}", self.cleared_frame);
        flush();
    }
}

pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
    flush();
}

pub fn flush() {
    io::stdout().flush().unwrap();
}
