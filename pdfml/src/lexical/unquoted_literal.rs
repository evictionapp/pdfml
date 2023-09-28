use crate::lexical::Skip;

use super::{
    error::{Error, ParseError},
    parse::Parse,
    Float, Ident, Integer,
};

#[derive(Debug, Clone)]
pub enum UnquotedLiteral<'a> {
    Integer(Integer<'a>),
    Float(Float<'a>),
    Ident(Ident<'a>),
}

impl<'a> Parse<'a> for UnquotedLiteral<'a> {
    fn parse(tokenizer: &'a retoken::Tokenizer) -> Result<Self, Error> {
        tokenizer.skip::<Skip>();

        use retoken::lazy_regex;
        let float_re = lazy_regex::regex!(r"^[0-9]+\.[0-9]+");
        let integer_re = lazy_regex::regex!("^[0-9]+");

        if tokenizer.peek_re(float_re) {
            return Ok(Self::Float(tokenizer.token().map_err(Self::error)?));
        }
        if tokenizer.peek_re(integer_re) {
            return Ok(Self::Integer(tokenizer.token().map_err(Self::error)?));
        }

        Ok(Self::Ident(tokenizer.token().map_err(Self::error)?))
    }

    fn error(error: retoken::Error<super::Alphabet>) -> Error {
        Error::Parse(ParseError {
            message: format!("Failed To Parse Unquoted Literal: {error}").into_boxed_str(),
            span: error.span(),
        })
    }
}
