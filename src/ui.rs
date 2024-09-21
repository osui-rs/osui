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
    fn run(&mut self, x: usize, y: usize) -> String {
        print!("\x1B[{};{}H", y + 2, x + 2);
        flush();
        let mut input = String::new();
        loop {
            match read_key().as_str() {
                key::ENTER => {
                    print!("\n\n");
                    return input;
                }
                key::BACKSPACE => {
                    if input.len() > 0 {
                        input.pop();
                        print!("\x1b[1D \x1b[1D");
                        flush();
                    }
                }
                c => {
                    if input.len() < self.max_length {
                        input += c.to_string().as_str();
                        print!("{c}");
                        flush();
                    }
                }
            }
        }
    }
}

pub fn input_box(max_length: usize) -> ComponentWrapper<Input> {
    ComponentWrapper::new(Input { max_length })
}
