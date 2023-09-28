use super::{
    bracketed_expr::BracketedExpr,
    error::{Error, ParseError},
    parse::Parse,
    Alphabet, Quoted,
};

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    Quoted(Quoted<'a>),
    Bracketed(BracketedExpr<'a>),
}

impl<'a> Parse<'a> for Expr<'a> {
    fn parse(tokenizer: &'a retoken::Tokenizer) -> Result<Self, Error> {
        if tokenizer.peek::<'a, Alphabet, Quoted>() {
            return Ok(Self::Quoted(tokenizer.token().map_err(Self::error)?));
        }

        Ok(Self::Bracketed(BracketedExpr::parse(tokenizer)?))
    }

    fn error(error: retoken::Error<super::Alphabet>) -> Error {
        Error::Parse(ParseError {
            message: format!("Failed To Parse Expression: {error}").into_boxed_str(),
            span: error.span(),
        })
    }
}
