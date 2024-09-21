use std::io::{self, Write};

pub mod ui;

#[derive(Clone)]
pub struct ComponentWrapper<T> {
    pub component: T,
    pub x: usize,
    pub y: usize,
    cleared_frame: String,
}

pub trait Component {
    fn render(&mut self) -> String {
        String::new()
    }
    fn run(&mut self, x: usize, y: usize) -> String {
        let _ = x;
        let _ = y;
        String::new()
    }
}

impl<T: Component> ComponentWrapper<T> {
    pub fn new(component: T) -> ComponentWrapper<T> {
        ComponentWrapper {
            component,
            x: 0,
            y: 0,
            cleared_frame: String::new(),
        }
    }
    pub fn render(&mut self) {
        self.clean();
        self.cleared_frame.clear();
        for (i, d) in format!("{}", self.component.render())
            .split("\n")
            .enumerate()
        {
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
    pub fn clean(&self) {
        print!("{}", self.cleared_frame);
        flush();
    }
    pub fn run(&mut self) -> String {
        self.component.run(self.x, self.y)
    }
}

pub fn clear() {
    print!("\x1b[H\x1b[2J\x1b[3J");
    flush();
}

pub fn flush() {
    io::stdout().flush().unwrap();
}
