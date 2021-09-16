#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("invalid SQLSTATE length: {0}")]
    InvalidLength(usize),
    #[error("unknown state: {0}")]
    UnknownState(String),
}
