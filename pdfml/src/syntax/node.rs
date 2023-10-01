use std::fmt::Display;

use retoken::{Span, Token};

use crate::lexical::{expr::Expr, markup_tag::MarkupType, tag::TagType, tags::Tags, Ident};

use super::{error::SyntaxError, tag_node::TagNode};

#[derive(Debug, Clone)]
pub enum NodeType<'a> {
    Expr(Expr<'a>),
    Tag(TagNode<'a>, Vec<Node<'a>>),
}

#[derive(Debug, Clone)]
pub struct Node<'a> {
    pub node_type: NodeType<'a>,
    pub span: Span,
}

impl<'a> Node<'a> {
    pub fn new(tags: &mut Tags<'a>) -> Result<Self, SyntaxError> {
        let tag = match tags.list.pop_front() {
            Some(tag) => tag,
            None => {
                return Err(SyntaxError {
                    message: "expected a tag or expression got end of source".into(),
                    span: tags.span.clone(),
                })
            }
        };

        match tag.tag_type {
            TagType::MarkupTag(markup_tag) => match markup_tag.markup_type {
                MarkupType::Open(open) => {
                    let children = match open.closing.is_with_slash() {
                        true => Vec::new(),
                        false => Self::parse_children(tags, open.ident.clone())?,
                    };

                    Ok(Node {
                        span: open.span.clone(),
                        node_type: NodeType::Tag(
                            TagNode {
                                ident: open.ident,
                                attributes: open.attributes,
                                span: open.span,
                            },
                            children,
                        ),
                    })
                }
                MarkupType::Close(close) => Err(SyntaxError {
                    message: format!("unexpected ending tag {}", close.ident.content())
                        .into_boxed_str(),
                    span: close.span,
                }),
            },
            TagType::Expr(expr) => Ok(Node {
                span: tag.span,
                node_type: NodeType::Expr(expr),
            }),
        }
    }

    fn parse_children(
        tags: &mut Tags<'a>,
        end_ident: Ident<'a>,
    ) -> Result<Vec<Node<'a>>, SyntaxError> {
        let mut children = Vec::new();

        loop {
            let tag = match tags.list.pop_front() {
                Some(tag) => tag,
                None => {
                    return Err(SyntaxError {
                        message: "expected a tag or expression got end of source".into(),
                        span: tags.span.clone(),
                    })
                }
            };

            match tag.tag_type {
                TagType::MarkupTag(markup_tag) => match markup_tag.markup_type {
                    MarkupType::Open(open) => {
                        let sub_children = match open.closing.is_with_slash() {
                            true => Vec::new(),
                            false => Self::parse_children(tags, open.ident.clone())?,
                        };

                        children.push(Node {
                            node_type: NodeType::Tag(
                                TagNode {
                                    ident: open.ident,
                                    attributes: open.attributes,
                                    span: open.span.clone(),
                                },
                                sub_children,
                            ),
                            span: open.span,
                        })
                    }
                    MarkupType::Close(close) => {
                        if close.ident.content() == end_ident.content() {
                            break;
                        }
                        return Err(SyntaxError {
                            message: format!("unexpected ending tag {}", close.ident.content())
                                .into_boxed_str(),
                            span: close.span,
                        });
                    }
                },
                TagType::Expr(expr) => children.push(Node {
                    span: tag.span,
                    node_type: NodeType::Expr(expr),
                }),
            }
        }

        Ok(children)
    }
}

impl<'a> Display for Node<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.node_type {
            NodeType::Expr(expr) => write!(f, "{}", expr),
            NodeType::Tag(tag, children) => {
                write!(f, "<{} {}>", tag.ident.content(), tag.attributes)?;
                for child in children {
                    write!(f, "{}", child)?;
                }
                write!(f, "</{}>", tag.ident.content())
            }
        }
    }
}
