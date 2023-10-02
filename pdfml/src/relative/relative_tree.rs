use crate::resolved::resolved_tree::ResolvedTree;

use super::{
    error::RelativeError,
    relative_rect::{DisplayType, Measurement, Pair, Quad, RelativeRect, TextContent},
};

#[derive(Debug, Clone)]
pub struct RelativeTree<'a> {
    pub relative_rect: RelativeRect,
    pub text_content: Option<TextContent<'a>>,
    pub children: Vec<RelativeTree<'a>>,
}

impl<'a> RelativeTree<'a> {
    pub fn new(resolved_tree: ResolvedTree<'a>) -> Result<Self, RelativeError> {
        resolved_tree.syntax_tree.list.into_iter().map()




        Ok(Self {
            relative_rect: RelativeRect {
                display_type: DisplayType::Row,
                dimensions: Pair {
                    x: Measurement::Mm(0.0),
                    y: Measurement::Mm(0.0),
                },
                padding: Quad::default(),
                border: Quad::default(),
            },
            text_content: None,
            children: Vec::new(),
        })
    }
}
