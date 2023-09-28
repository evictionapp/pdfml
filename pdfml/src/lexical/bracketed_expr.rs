use retoken::Span;

use super::{
    call_expr::CallExpr,
    error::{Error, ParseError},
    parse::Parse,
    unquoted_literal::UnquotedLiteral,
    Alphabet, CloseBracket, OpenBracket, OpenParen,
};

#[derive(Debug, Clone)]
pub enum TemplateExpr<'a> {
    UnquotedLiteral(UnquotedLiteral<'a>),
    CallExpr(CallExpr<'a>),
}

#[derive(Debug, Clone)]
pub struct BracketedExpr<'a> {
    pub open_bracket: OpenBracket,
    pub template_expr: TemplateExpr<'a>,
    pub close_bracket: CloseBracket,
    pub span: Span,
}

impl<'a> Parse<'a> for BracketedExpr<'a> {
    fn parse(tokenizer: &'a retoken::Tokenizer) -> Result<Self, Error> {
        let start = tokenizer.cursor();

        let open_bracket = tokenizer.token().map_err(Self::error)?;

        let unquoted = UnquotedLiteral::parse(tokenizer)?;

        let template_expr = match unquoted {
            UnquotedLiteral::Ident(ident) => match tokenizer.peek::<'a, Alphabet, OpenParen>() {
                true => TemplateExpr::CallExpr(CallExpr::new(ident, tokenizer)?),
                false => TemplateExpr::UnquotedLiteral(UnquotedLiteral::Ident(ident)),
            },
            _ => TemplateExpr::UnquotedLiteral(unquoted),
        };
        let close_bracket = tokenizer.token().map_err(Self::error)?;

        let end = tokenizer.cursor();

        let span = Span { start, end };

        Ok(Self {
            open_bracket,
            template_expr,
            close_bracket,
            span,
        })
    }

    fn error(error: retoken::Error<super::Alphabet>) -> Error {
        Error::Parse(ParseError {
            message: format!("Failed To Parse Bracketed Expression: {error}").into_boxed_str(),
            span: error.span(),
        })
    }
}
