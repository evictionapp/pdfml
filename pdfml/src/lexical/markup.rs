use retoken::Span;

use super::{expr::Expr, tag::Tag};

#[derive(Debug, Clone)]
pub enum MarkupType<'a> {
    Tag(Tag<'a>),
    Expr(Expr<'a>),
}

#[derive(Debug, Clone)]
pub struct Markup<'a> {
    pub markup_type: MarkupType<'a>,
    pub span: Span,
}
