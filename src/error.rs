pub type Result<T> = std::result::Result<T, OsuiError>;

#[derive(Debug)]
pub enum OsuiError {
    Io(std::io::Error),
}

impl From<std::io::Error> for OsuiError {
    fn from(e: std::io::Error) -> Self {
        OsuiError::Io(e)
    }
}
