pub(crate) mod utils {
    //! The `utils` module provides utility functions for terminal manipulation
    //! and string operations, designed to enhance terminal-based applications.

    use std::io::{self, Write};

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
}

pub trait Renderer {
    fn draw_text(&mut self, text: &str, x: u32, y: u32) -> crate::Result<()>;
}

pub struct OsuiRenderer;

impl Renderer for OsuiRenderer {
    fn draw_text(&mut self, text: &str, x: u32, y: u32) -> crate::Result<()> {
        for line in text.lines() {
            print!("\x1b[{};{}H{line}\x1b[0m", y + 1, x + 1);
        }
        utils::flush()?;
        Ok(())
    }
}
