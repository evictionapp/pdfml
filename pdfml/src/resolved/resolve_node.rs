use std::collections::HashMap;

use crate::syntax::node::{Node, NodeType};

use super::{error::ResolvedError, resolve_expr::resolve_expr, resolve_tag::resolve_tag};

pub fn resolve_node<'a>(
    mut node: Node<'a>,
    ident_map: &mut HashMap<Box<str>, Box<str>>,
) -> Result<Node<'a>, ResolvedError> {
    node.node_type = match node.node_type {
        NodeType::Expr(expr) => NodeType::Expr(resolve_expr(expr, ident_map)?),
        NodeType::Tag(tag, children) => NodeType::Tag(
            resolve_tag(tag, ident_map)?,
            children
                .into_iter()
                .map(|node| resolve_node(node, ident_map))
                .collect::<Result<_, _>>()?,
        ),
    };

    Ok(node)
}
