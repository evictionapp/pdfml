use thiserror::Error;

use crate::{
    lexical::error::LexicalError, resolved::error::ResolvedError, syntax::error::SyntaxError,
};

#[derive(Debug, Error)]
pub enum RenderError {
    #[error("{0}")]
    Syntax(#[from] SyntaxError),
    #[error("{0}")]
    Lexical(#[from] LexicalError),
    #[error("{0}")]
    Resolved(#[from] ResolvedError),
}
