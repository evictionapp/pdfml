use retoken::Span;

use super::{error::Error, markup_close::MarkupClose, markup_open::MarkupOpen, parse::Parse, Skip};

#[derive(Debug, Clone)]
pub enum MarkupType<'a> {
    Open(MarkupOpen<'a>),
    Close(MarkupClose<'a>),
}

#[derive(Debug, Clone)]
pub struct MarkupTag<'a> {
    pub markup_type: MarkupType<'a>,
    pub span: Span,
}

impl<'a> Parse<'a> for MarkupTag<'a> {
    fn parse(tokenizer: &'a retoken::Tokenizer) -> Result<Self, Error> {
        let start = tokenizer.cursor();

        use retoken::lazy_regex;
        let re = lazy_regex::regex!(r"^\s*<\s*/");

        tokenizer.skip::<Skip>();
        let markup_type = match tokenizer.peek_re(re) {
            true => MarkupType::Close(MarkupClose::parse(tokenizer)?),
            false => MarkupType::Open(MarkupOpen::parse(tokenizer)?),
        };

        let end = tokenizer.cursor();

        let span = Span { start, end };

        Ok(Self { markup_type, span })
    }

    fn error(error: retoken::Error<super::Alphabet>) -> Error {
        Error::Token(error)
    }
}
