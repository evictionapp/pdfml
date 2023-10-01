use retoken::{SliceError, Span};
use thiserror::Error;

use super::Alphabet;

#[derive(Debug, Error)]
#[error("{message} @ {span:?}")]
pub struct ParseError {
    pub message: Box<str>,
    pub span: Span,
}

#[derive(Debug, Error)]
pub enum LexicalError {
    #[error("{0}")]
    Token(retoken::Error<Alphabet>),
    #[error("{0}")]
    Parse(#[from] ParseError),
}

impl From<SliceError> for LexicalError {
    fn from(value: SliceError) -> Self {
        Self::Token(retoken::Error::Slice(value))
    }
}
