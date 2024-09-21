use crate::{flush, key, read_key, Component, ComponentWrapper};
impl Component for String {
    fn render(&self) -> String {
        self.clone()
    }
}

pub fn text(text: &str) -> ComponentWrapper<String> {
    ComponentWrapper::new(text.to_string())
}

#[derive(Clone)]
pub struct Input {
    pub max_length: usize,
    pub key: String,
    pub data: String,
}

impl Component for Input {
    fn render(&self) -> String {
        format!(
            " {}\n|{}|\n {}",
            "_".repeat(self.max_length),
            " ".repeat(self.max_length),
            "‾".repeat(self.max_length),
        )
    }
    fn run(&mut self, x: usize, y: usize, update_fn: fn(&mut Self) -> bool) -> String {
        print!("\x1B[{};{}H", y + 2, x + 2);
        flush();
        loop {
            self.key = read_key();
            if update_fn(self) {
                break;
            }
            match self.key.as_str() {
                key::ENTER => break,
                key::BACKSPACE => {
                    if self.data.len() > 0 {
                        self.data.pop();
                        print!("\x1b[1D \x1b[1D");
                        flush();
                    }
                }
                c => {
                    if self.data.len() < self.max_length {
                        self.data += c.to_string().as_str();
                        print!("{c}");
                        flush();
                    }
                }
            }
        }
        print!("\n\n");
        return self.data.clone();
    }
}

pub fn input_box(max_length: usize) -> ComponentWrapper<Input> {
    ComponentWrapper::new(Input {
        max_length,
        key: String::new(),
        data: String::new(),
    })
}
