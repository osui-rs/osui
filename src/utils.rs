use std::io::Write;

pub fn clear() -> crate::Result<()> {
    print!("\x1B[2J\x1B[H");
    std::io::stdout().flush()
}

pub fn hide_cursor() -> crate::Result<()> {
    print!("\x1b[?25l");
    std::io::stdout().flush()
}

pub fn show_cursor() -> crate::Result<()> {
    print!("\x1B[?25h");
    std::io::stdout().flush()
}

pub fn flush() -> crate::Result<()> {
    std::io::stdout().flush()
}

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
