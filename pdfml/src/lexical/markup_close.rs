use retoken::Span;

use super::{
    error::{LexicalError, ParseError},
    parse::Parse,
    CloseAngle, Ident, OpenAngle, Slash,
};

#[derive(Debug, Clone)]
pub struct MarkupClose<'a> {
    pub open_angle: OpenAngle,
    pub slash: Slash,
    pub ident: Ident<'a>,
    pub close_angle: CloseAngle,
    pub span: Span,
}

impl<'a> Parse<'a> for MarkupClose<'a> {
    fn parse(tokenizer: &'a retoken::Tokenizer) -> Result<Self, LexicalError> {
        let start = tokenizer.cursor();

        let open_angle = tokenizer.token().map_err(Self::error)?;
        let slash = tokenizer.token().map_err(Self::error)?;
        let ident = tokenizer.token().map_err(Self::error)?;
        let close_angle = tokenizer.token().map_err(Self::error)?;

        let end = tokenizer.cursor();

        let span = Span { start, end };

        Ok(Self {
            open_angle,
            slash,
            ident,
            close_angle,
            span,
        })
    }

    fn error(error: retoken::Error<super::Alphabet>) -> LexicalError {
        LexicalError::Parse(ParseError {
            message: format!("Failed To Parse Closing Markup Tag: {error}").into_boxed_str(),
            span: error.span(),
        })
    }
}
