use super::{
    error::{Error, ParseError},
    parse::Parse,
    unquoted_literal::UnquotedLiteral,
    Alphabet, Quoted,
};

#[derive(Debug, Clone)]
pub enum Literal<'a> {
    Quoted(Quoted<'a>),
    Unquoted(UnquotedLiteral<'a>),
}

impl<'a> Parse<'a> for Literal<'a> {
    fn parse(tokenizer: &'a retoken::Tokenizer) -> Result<Self, Error> {
        if tokenizer.peek::<'a, Alphabet, Quoted>() {
            return Ok(Self::Quoted(tokenizer.token().map_err(Self::error)?));
        }

        Ok(Self::Unquoted(UnquotedLiteral::parse(tokenizer)?))
    }

    fn error(error: retoken::Error<super::Alphabet>) -> Error {
        Error::Parse(ParseError {
            message: format!("Failed To Parse Literal: {error}").into_boxed_str(),
            span: error.span(),
        })
    }
}
