#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("invalid SQLSTATE length: {0}")]
    InvalidLength(usize),
}
