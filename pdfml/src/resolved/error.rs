use retoken::Span;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("expected ident {ident} at {span:?}")]
pub struct ExpectedIdentError {
    pub ident: Box<str>,
    pub span: Span,
}

#[derive(Debug, Error)]
pub enum ResolvedError {
    #[error("unused ident {0}")]
    UnusedIdent(Box<str>),
    #[error("{0}")]
    ExpectedIdent(ExpectedIdentError),
}
