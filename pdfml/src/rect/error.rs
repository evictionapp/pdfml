use retoken::Span;
use thiserror::Error;

#[derive(Debug)]
pub enum AttributeErrorType {
    Expected,
    Unexpected,
}

#[derive(Debug, Error)]
#[error("attribute error @ {}. {error_type:?} attribute {attribute}", .span.start )]
pub struct AttributeError {
    pub attribute: String,
    pub error_type: AttributeErrorType,
    pub span: Span,
}

#[derive(Debug, Error)]
pub enum RectError {
    #[error("{0}")]
    Attribute(AttributeError),
}
