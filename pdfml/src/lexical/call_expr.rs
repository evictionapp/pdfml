use retoken::{Span, Tokenizer};

use super::{
    arguments::Arguments,
    error::{Error, ParseError},
    parse::Parse,
    Alphabet, CloseParen, Ident, OpenParen,
};

#[derive(Debug, Clone)]
pub struct CallExpr<'a> {
    pub function_name: Ident<'a>,
    pub open_paren: OpenParen,
    pub arguments: Arguments<'a>,
    pub close_paren: CloseParen,
    pub span: Span,
}

impl<'a> CallExpr<'a> {
    pub fn new(ident: Ident<'a>, tokenizer: &'a Tokenizer) -> Result<Self, Error> {
        let open_paren = tokenizer.token().map_err(Self::error)?;
        let arguments = Arguments::parse(tokenizer)?;
        let close_paren = tokenizer.token().map_err(Self::error)?;

        let span = Span {
            start: ident.span.start,
            end: tokenizer.cursor(),
        };

        Ok(Self {
            function_name: ident,
            open_paren,
            arguments,
            close_paren,
            span,
        })
    }

    fn error(error: retoken::Error<Alphabet>) -> Error {
        Error::Parse(ParseError {
            message: format!("Failed To Parse Function Call: {error}").into_boxed_str(),
            span: error.span(),
        })
    }
}
