use retoken::Span;

use super::{arg::Arg, parse::Parse, Alphabet, CloseParen};

#[derive(Debug, Clone)]
pub struct Arguments<'a> {
    pub list: Vec<Arg<'a>>,
    pub span: Span,
}

impl<'a> Parse<'a> for Arguments<'a> {
    fn parse(tokenizer: &'a retoken::Tokenizer) -> Result<Self, super::error::Error> {
        let start = tokenizer.cursor();

        let mut list = Vec::new();

        while !tokenizer.peek::<'a, Alphabet, CloseParen>() {
            list.push(Arg::parse(tokenizer)?);
        }

        let end = tokenizer.cursor();

        let span = Span { start, end };

        Ok(Self { list, span })
    }

    fn error(error: retoken::Error<super::Alphabet>) -> super::error::Error {
        super::error::Error::Token(error)
    }
}
