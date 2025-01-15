use std::io::Write;

pub fn clear() -> crate::Result<()> {
    print!("\x1B[2J\x1B[H");
    std::io::stdout().flush()?;
    Ok(())
}

pub fn hide_cursor() -> crate::Result<()> {
    print!("\x1b[?25l");
    std::io::stdout().flush()?;
    Ok(())
}

pub fn show_cursor() -> crate::Result<()> {
    print!("\x1B[?25h");
    std::io::stdout().flush()?;
    Ok(())
}

pub fn flush() -> crate::Result<()> {
    std::io::stdout().flush()?;
    Ok(())
}
