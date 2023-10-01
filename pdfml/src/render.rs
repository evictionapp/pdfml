use std::collections::HashMap;

use retoken::Tokenizer;

use crate::{
    lexical::{parse::Parse, tags::Tags},
    render::error::RenderError,
    resolved::resolved_tree::ResolvedTree,
    syntax::syntax_tree::SyntaxTree,
};

pub mod error;

pub fn render(src: &str, ident_map: HashMap<Box<str>, Box<str>>) -> Result<(), RenderError> {
    let tokenizer = Tokenizer::new(src);

    let tags = Tags::parse(&tokenizer)?;

    let tree = SyntaxTree::new(tags)?;

    let resolved = ResolvedTree::new(tree, ident_map)?;

    println!("{:?}", resolved);

    Ok(())
}
