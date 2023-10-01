use retoken::Span;

use super::{
    error::{LexicalError, ParseError},
    parse::Parse,
    CloseBracket, Ident, OpenBracket,
};

#[derive(Debug, Clone)]
pub struct BracketedExpr<'a> {
    pub open_bracket: OpenBracket,
    pub ident: Ident<'a>,
    pub close_bracket: CloseBracket,
    pub span: Span,
}

impl<'a> Parse<'a> for BracketedExpr<'a> {
    fn parse(tokenizer: &'a retoken::Tokenizer) -> Result<Self, LexicalError> {
        let start = tokenizer.cursor();

        let open_bracket = tokenizer.token().map_err(Self::error)?;
        let ident = tokenizer.token().map_err(Self::error)?;
        let close_bracket = tokenizer.token().map_err(Self::error)?;

        let end = tokenizer.cursor();

        let span = Span { start, end };

        Ok(Self {
            open_bracket,
            ident,
            close_bracket,
            span,
        })
    }

    fn error(error: retoken::Error<super::Alphabet>) -> LexicalError {
        LexicalError::Parse(ParseError {
            message: format!("Failed To Parse Bracketed Expression: {error}").into_boxed_str(),
            span: error.span(),
        })
    }
}
