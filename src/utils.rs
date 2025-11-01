//! The `utils` module provides utility functions for terminal manipulation
//! and string operations, designed to enhance terminal-based applications.

use std::io::{self, Write};

use crate::style::RawTransform;

/// Clears the terminal screen and moves the cursor to the top-left corner.
///
/// # Returns
/// A `io::Result<()>` indicating whether the operation succeeded.
///
/// # Example
/// ```
/// utils::clear().unwrap();
/// ```
pub fn clear() -> io::Result<()> {
    print!("\x1B[2J\x1B[H");
    std::io::stdout().flush()
}

/// Hides the terminal cursor.
///
/// # Returns
/// A `io::Result<()>` indicating whether the operation succeeded.
///
/// # Example
/// ```
/// utils::hide_cursor().unwrap();
/// ```
pub fn hide_cursor() -> io::Result<()> {
    print!("\x1b[?25l");
    std::io::stdout().flush()
}

/// Shows the terminal cursor.
///
/// # Returns
/// A `io::Result<()>` indicating whether the operation succeeded.
///
/// # Example
/// ```
/// utils::show_cursor().unwrap();
/// ```
pub fn show_cursor() -> io::Result<()> {
    print!("\x1B[?25h");
    std::io::stdout().flush()
}

/// Flushes the terminal's stdout buffer.
///
/// # Returns
/// A `io::Result<()>` indicating whether the operation succeeded.
///
/// # Example
/// ```
/// utils::flush().unwrap();
/// ```
pub fn flush() -> io::Result<()> {
    std::io::stdout().flush()
}

/// Calculates the width and height of a string when rendered in a terminal.
///
/// # Arguments
/// - `s`: The input string.
///
/// # Returns
/// A tuple `(u16, u16)` where:
/// - The first value is the maximum width of the string in characters.
/// - The second value is the height of the string in lines.
///
/// # Example
/// ```
/// let (width, height) = utils::str_size("Hello\nWorld!");
/// assert_eq!((5, 2), (width, height));
/// ```
pub fn str_size(s: &str) -> (u16, u16) {
    let mut height = 1;
    let mut max_width = 0;
    let mut current_width = 0;

    for b in s.bytes() {
        if b == b'\n' {
            height += 1;
            max_width = max_width.max(current_width);
            current_width = 0;
        } else {
            current_width += 1;
        }
    }

    max_width = max_width.max(current_width);

    (max_width, height)
}

pub fn hex_ansi(hex: u32) -> String {
    let r = ((hex >> 16) & 0xFF) as u8;
    let g = ((hex >> 8) & 0xFF) as u8;
    let b = (hex & 0xFF) as u8;
    format!("\x1b[38;2;{r};{g};{b}m")
}

pub fn hex_ansi_bg(hex: u32) -> String {
    let r = ((hex >> 16) & 0xFF) as u8;
    let g = ((hex >> 8) & 0xFF) as u8;
    let b = (hex & 0xFF) as u8;
    format!("\x1b[48;2;{r};{g};{b}m")
}

pub(crate) fn print(x: u16, y: u16, text: &str, parent_transform: &RawTransform) {
    for (i, line) in text.lines().enumerate() {
        if y + i as u16 >= parent_transform.height + parent_transform.y {
            break;
        }
        print!("\x1b[{};{}H{line}\x1b[0m", y + i as u16 + 1, x + 1);
        flush().unwrap();
    }
}

pub(crate) fn print_liner(
    x: u16,
    y: u16,
    liner: &str,
    text: &str,
    parent_transform: &RawTransform,
) {
    for (i, line) in text.lines().enumerate() {
        if y + i as u16 >= parent_transform.height + parent_transform.y {
            break;
        }
        print!("\x1b[{};{}H{liner}{line}\x1b[0m", y + i as u16 + 1, x + 1);
        flush().unwrap();
    }
}
