use retoken::Span;

use super::{
    attributes::Attributes,
    closing::Closing,
    error::{LexicalError, ParseError},
    parse::Parse,
    Ident, OpenAngle,
};

#[derive(Debug, Clone)]
pub struct MarkupOpen<'a> {
    pub open_angle: OpenAngle,
    pub ident: Ident<'a>,
    pub attributes: Attributes<'a>,
    pub closing: Closing,
    pub span: Span,
}

impl<'a> Parse<'a> for MarkupOpen<'a> {
    fn parse(tokenizer: &'a retoken::Tokenizer) -> Result<Self, LexicalError> {
        let start = tokenizer.cursor();

        let open_angle = tokenizer.token().map_err(Self::error)?;
        let ident = tokenizer.token().map_err(Self::error)?;
        let attributes = Attributes::parse(tokenizer)?;
        let closing = Closing::parse(tokenizer)?;

        let end = tokenizer.cursor();

        let span = Span { start, end };

        Ok(Self {
            open_angle,
            ident,
            attributes,
            closing,
            span,
        })
    }

    fn error(error: retoken::Error<super::Alphabet>) -> LexicalError {
        LexicalError::Parse(ParseError {
            message: format!("Failed To Parse Opening Markup Tag: {error}").into_boxed_str(),
            span: error.span(),
        })
    }
}
