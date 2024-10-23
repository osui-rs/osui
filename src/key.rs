use std::io::{self, Read};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum KeyKind {
    Enter,
    Tab,
    ShiftTab,
    Escape,
    Up,
    Down,
    Left,
    Right,
    Char(String),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Key {
    pub kind: KeyKind,
    pub raw: String,
}

impl Key {
    pub fn new(k: String) -> Key {
        Key {
            raw: k.clone(),
            kind: match k.as_str() {
                "\r" => KeyKind::Enter,
                "\t" => KeyKind::Tab,
                "\x1b[Z" => KeyKind::ShiftTab,
                "\x1b" => KeyKind::Escape,
                "\x1b[A" => KeyKind::Up,
                "\x1b[B" => KeyKind::Down,
                "\x1b[C" => KeyKind::Right,
                "\x1b[D" => KeyKind::Left,

                _ => KeyKind::Char(k.clone()),
            },
        }
    }
}

pub fn read_key() -> Key {
    let mut buffer = vec![0; 3];
    io::stdin().read(&mut buffer).unwrap();
    Key::new(
        String::from_utf8(buffer)
            .unwrap()
            .trim_matches('\0')
            .to_string(),
    )
}
