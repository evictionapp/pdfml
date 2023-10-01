use std::fmt::Display;

use retoken::Token;

use super::{
    bracketed_expr::BracketedExpr,
    error::{LexicalError, ParseError},
    parse::Parse,
    Alphabet, Quoted,
};

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    Quoted(Quoted<'a>),
    Bracketed(BracketedExpr<'a>),
    Resolved(Box<str>),
}

impl<'a> Parse<'a> for Expr<'a> {
    fn parse(tokenizer: &'a retoken::Tokenizer) -> Result<Self, LexicalError> {
        if tokenizer.peek::<'a, Alphabet, Quoted>() {
            return Ok(Self::Quoted(Quoted::token(tokenizer).map_err(Self::error)?));
        }

        Ok(Self::Bracketed(BracketedExpr::parse(tokenizer)?))
    }

    fn error(error: retoken::Error<super::Alphabet>) -> LexicalError {
        LexicalError::Parse(ParseError {
            message: format!("Failed To Parse Expression: {error}").into_boxed_str(),
            span: error.span(),
        })
    }
}

impl<'a> Display for Expr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Quoted(quoted) => write!(f, "{:?}", quoted.content()),
            Self::Bracketed(bracketed) => write!(f, "{{ {} }}", bracketed.ident.content()),
            Self::Resolved(resolved) => write!(f, "{:?}", resolved),
        }
    }
}
