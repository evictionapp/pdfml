use retoken::Span;

use crate::lexical::{attributes::Attributes, Ident};

#[derive(Debug, Clone)]
pub struct TagNode<'a> {
    pub ident: Ident<'a>,
    pub attributes: Attributes<'a>,
    pub span: Span,
}
