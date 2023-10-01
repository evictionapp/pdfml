use retoken::Span;

use super::{
    error::LexicalError, expr::Expr, markup_tag::MarkupTag, parse::Parse, Alphabet, OpenAngle, Skip,
};

#[derive(Debug, Clone)]
pub enum TagType<'a> {
    MarkupTag(MarkupTag<'a>),
    Expr(Expr<'a>),
}

#[derive(Debug, Clone)]
pub struct Tag<'a> {
    pub tag_type: TagType<'a>,
    pub span: Span,
}

impl<'a> Parse<'a> for Tag<'a> {
    fn parse(tokenizer: &'a retoken::Tokenizer) -> Result<Self, LexicalError> {
        tokenizer.skip::<Skip>();

        let start = tokenizer.cursor();

        let tag_type = match tokenizer.peek::<'a, Alphabet, OpenAngle>() {
            true => TagType::MarkupTag(MarkupTag::parse(tokenizer)?),
            false => TagType::Expr(Expr::parse(tokenizer)?),
        };

        let end = tokenizer.cursor();

        let span = Span { start, end };

        Ok(Self { tag_type, span })
    }

    fn error(error: retoken::Error<super::Alphabet>) -> LexicalError {
        LexicalError::Token(error)
    }
}
