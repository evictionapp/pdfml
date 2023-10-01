use retoken::Span;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("syntax error {message} at {span:?}")]
pub struct SyntaxError {
    pub message: Box<str>,
    pub span: Span,
}
