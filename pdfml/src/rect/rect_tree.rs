use std::collections::HashMap;

use crate::syntax::{node::Node, syntax_tree::SyntaxTree};

use super::{dimensions::Dimensions, error::RectError};

pub struct RectTree<'a> {
    pub dimensions: Dimensions,
    pub node_type: Node<'a>,
    pub children: Vec<RectTree<'a>>,
}

impl<'a> RectTree<'a> {
    pub fn new(
        syntax_tree: SyntaxTree<'a>,
        ident_map: HashMap<Box<str>, Box<str>>,
    ) -> Result<Self, RectError> {
        todo!()
    }
}
