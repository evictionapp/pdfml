use std::collections::VecDeque;

use retoken::Span;

use super::{error::LexicalError, parse::Parse, tag::Tag, Skip};

#[derive(Debug, Clone)]
pub struct Tags<'a> {
    pub list: VecDeque<Tag<'a>>,
    pub span: Span,
}

impl<'a> Parse<'a> for Tags<'a> {
    fn parse(tokenizer: &'a retoken::Tokenizer) -> Result<Self, super::error::LexicalError> {
        let start = tokenizer.cursor();
        let mut list = VecDeque::new();

        while !tokenizer.slice_with_cursor()?.0.is_empty() {
            list.push_back(Tag::parse(tokenizer)?);

            tokenizer.skip::<Skip>();
        }

        let span = Span {
            start,
            end: tokenizer.cursor(),
        };

        Ok(Self { list, span })
    }

    fn error(error: retoken::Error<super::Alphabet>) -> LexicalError {
        LexicalError::Token(error)
    }
}
