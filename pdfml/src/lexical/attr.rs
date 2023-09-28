use retoken::Span;

use super::{
    error::{Error, ParseError},
    expr::Expr,
    parse::Parse,
    Equal, Ident, Skip,
};

#[derive(Debug, Clone)]
pub struct Attr<'a> {
    pub ident: Ident<'a>,
    pub equal: Equal,
    pub expr: Expr<'a>,
    pub span: Span,
}

impl<'a> Parse<'a> for Attr<'a> {
    fn parse(tokenizer: &'a retoken::Tokenizer) -> Result<Self, Error> {
        let start = tokenizer.cursor();

        let ident = tokenizer.token().map_err(Self::error)?;
        let equal = tokenizer.token().map_err(Self::error)?;
        tokenizer.skip::<Skip>();
        let expr = Expr::parse(tokenizer)?;

        let end = tokenizer.cursor();

        let span = Span { start, end };

        Ok(Self {
            ident,
            equal,
            expr,
            span,
        })
    }

    fn error(error: retoken::Error<super::Alphabet>) -> Error {
        Error::Parse(ParseError {
            message: format!("Failed To Parse Attribute: {error}").into_boxed_str(),
            span: error.span(),
        })
    }
}
