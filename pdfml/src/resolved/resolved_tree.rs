use std::collections::HashMap;

use crate::syntax::syntax_tree::SyntaxTree;

use super::{error::ResolvedError, resolve_node::resolve_node};

#[derive(Debug, Clone)]
pub struct ResolvedTree<'a> {
    pub syntax_tree: SyntaxTree<'a>,
}

impl<'a> ResolvedTree<'a> {
    pub fn new(
        syntax_tree: SyntaxTree<'a>,
        mut ident_map: HashMap<Box<str>, Box<str>>,
    ) -> Result<Self, ResolvedError> {
        let syntax_tree = SyntaxTree {
            list: syntax_tree
                .list
                .into_iter()
                .map(|node| resolve_node(node, &mut ident_map))
                .collect::<Result<_, _>>()?,
        };

        match ident_map.keys().next() {
            Some(key) => Err(ResolvedError::UnusedIdent(key.clone())),
            None => Ok(Self { syntax_tree }),
        }
    }
}
