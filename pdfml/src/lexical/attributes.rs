use retoken::Span;

use super::{attr::Attr, parse::Parse, Alphabet, CloseAngle, Slash};

#[derive(Debug, Clone)]
pub struct Attributes<'a> {
    pub list: Vec<Attr<'a>>,
    pub span: Span,
}

impl<'a> Parse<'a> for Attributes<'a> {
    fn parse(tokenizer: &'a retoken::Tokenizer) -> Result<Self, super::error::Error> {
        let start = tokenizer.cursor();

        let mut list = Vec::new();

        while !tokenizer.peek::<'a, Alphabet, Slash>()
            && !tokenizer.peek::<'a, Alphabet, CloseAngle>()
        {
            list.push(Attr::parse(tokenizer)?);
        }

        let end = tokenizer.cursor();

        let span = Span { start, end };

        Ok(Self { list, span })
    }

    fn error(error: retoken::Error<super::Alphabet>) -> super::error::Error {
        super::error::Error::Token(error)
    }
}
