use std::io::{self, Read};

/// Represents different key inputs that can be detected from the terminal.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Key {
    /// Represents the Enter key (carriage return).
    Enter,
    /// Represents the Tab key.
    Tab,
    /// Represents the Shift+Tab key combination.
    ShiftTab,
    /// Represents the Escape key.
    Escape,
    /// Represents the Up arrow key.
    Up,
    /// Represents the Down arrow key.
    Down,
    /// Represents the Left arrow key.
    Left,
    /// Represents the Right arrow key.
    Right,
    /// Represents any other character or sequence that does not match a predefined key.
    Other(String),
}

impl Key {
    /// Creates a new `Key` instance based on the input string.
    ///
    /// This function matches specific strings with known keys (e.g., arrow keys, Enter, Tab)
    /// and returns the corresponding `Key` variant. If the string does not match any of the
    /// predefined keys, it returns `Key::Other` with the string content.
    ///
    /// # Arguments
    ///
    /// * `k` - A `String` representing the raw key input.
    ///
    /// # Returns
    ///
    /// A `Key` enum variant corresponding to the input.
    pub fn new(k: String) -> Key {
        match k.as_str() {
            "\r" => Key::Enter,
            "\t" => Key::Tab,
            "\x1b[Z" => Key::ShiftTab,
            "\x1b" => Key::Escape,
            "\x1b[A" => Key::Up,
            "\x1b[B" => Key::Down,
            "\x1b[C" => Key::Right,
            "\x1b[D" => Key::Left,
            _ => Key::Other(k),
        }
    }
}

/// Reads a key from standard input and returns it as a `Key` enum variant.
///
/// This function calls `read_key_raw` to get the raw key input as a `String`,
/// then uses `Key::new` to convert it to a `Key` variant.
///
/// # Returns
///
/// A `Key` enum variant corresponding to the input read from standard input.
pub fn read_key() -> Key {
    Key::new(read_key_raw())
}

/// Reads raw key input from standard input as a UTF-8 encoded string.
///
/// This function attempts to read up to 3 bytes from standard input and
/// returns them as a `String`. It uses a buffer size of 3, which is enough
/// to capture most common key sequences, including basic escape sequences
/// for arrow keys and other special keys.
///
/// # Returns
///
/// A `String` representing the raw key input read from standard input.
///
/// # Panics
///
/// This function will panic if reading from `stdin` fails or if the bytes
/// cannot be converted to a valid UTF-8 `String`.
pub fn read_key_raw() -> String {
    let mut buffer = vec![0; 3];
    io::stdin().read(&mut buffer).unwrap();
    String::from_utf8(buffer)
        .unwrap()
        .trim_matches('\0')
        .to_string()
}
