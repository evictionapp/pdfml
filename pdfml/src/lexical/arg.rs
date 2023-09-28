use retoken::Span;

use super::{
    error::{Error, ParseError},
    literal::Literal,
    parse::Parse,
    Alphabet, Comma,
};

#[derive(Debug, Clone)]
pub struct Arg<'a> {
    pub ident: Literal<'a>,
    pub comma: Option<Comma>,
    pub span: Span,
}

impl<'a> Parse<'a> for Arg<'a> {
    fn parse(tokenizer: &'a retoken::Tokenizer) -> Result<Self, Error> {
        let start = tokenizer.cursor();

        let ident = Literal::parse(tokenizer)?;

        let comma = match tokenizer.peek::<'a, Alphabet, Comma>() {
            true => Some(tokenizer.token().map_err(Self::error)?),
            false => None,
        };

        let end = tokenizer.cursor();

        let span = Span { start, end };

        Ok(Self { ident, comma, span })
    }

    fn error(error: retoken::Error<super::Alphabet>) -> Error {
        Error::Parse(ParseError {
            message: format!("Failed To Parse Argument: {error}").into_boxed_str(),
            span: error.span(),
        })
    }
}
