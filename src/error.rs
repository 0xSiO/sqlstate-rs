#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("invalid SQLSTATE length: {0}")]
    InvalidLength(usize),
    #[error("unknown subclass: '{0}'")]
    UnknownSubclass(String),
}
