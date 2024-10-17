use std::io::{self, Read};

#[derive(Debug, Clone, PartialEq)]
pub enum KeyKind {
    Enter,
    Escape,
    Up,
    Down,
    Left,
    Right,
    Char(String),
}

#[derive(Debug, Clone)]
pub struct Key {
    pub kind: KeyKind,
    pub raw: String,
    pub ctrl: bool,
    pub shift: bool,
}

impl Key {
    pub fn new(k: String) -> Key {
        let mut ctrl = false;
        let mut shift = false;
        let kind = match k.as_str() {
            "\r" => KeyKind::Enter,
            "\x1b" => KeyKind::Escape,
            "\x1b[A" => KeyKind::Up,
            "\x1b[B" => KeyKind::Down,
            "\x1b[C" => KeyKind::Right,
            "\x1b[D" => KeyKind::Left,

            // Ctrl keys
            "\u{17}" => {
                ctrl = true;
                KeyKind::Char("w".to_string())
            }

            "\u{13}" => {
                ctrl = true;
                KeyKind::Char("s".to_string())
            }

            "\u{1}" => {
                ctrl = true;
                KeyKind::Char("a".to_string())
            }

            "\u{4}" => {
                ctrl = true;
                KeyKind::Char("d".to_string())
            }

            "\u{11}" => {
                ctrl = true;
                KeyKind::Char("q".to_string())
            }

            _ => {
                shift = k.to_uppercase() == k;
                KeyKind::Char(k.clone())
            }
        };
        Key {
            raw: k,
            kind,
            ctrl,
            shift,
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
