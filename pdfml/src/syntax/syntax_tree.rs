use std::fmt::Display;

use crate::lexical::tags::Tags;

use super::{error::SyntaxError, node::Node};

#[derive(Debug, Clone)]
pub struct SyntaxTree<'a> {
    pub list: Vec<Node<'a>>,
}

impl<'a> SyntaxTree<'a> {
    pub fn new(mut tags: Tags<'a>) -> Result<Self, SyntaxError> {
        let mut list = Vec::new();

        while !tags.list.is_empty() {
            list.push(Node::new(&mut tags)?)
        }

        Ok(Self { list })
    }
}

impl<'a> Display for SyntaxTree<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for node in self.list.iter() {
            write!(f, "{}", node)?;
        }
        Ok(())
    }
}
