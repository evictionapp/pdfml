use std::collections::HashMap;

use retoken::Tokenizer;

use crate::{
    lexical::{parse::Parse, tags::Tags},
    relative::relative_tree::RelativeTree,
    render::error::RenderError,
    resolved::resolved_tree::ResolvedTree,
    syntax::syntax_tree::SyntaxTree,
};

pub mod error;

pub fn render(src: &str, ident_map: HashMap<Box<str>, Box<str>>) -> Result<(), RenderError> {
    let tokenizer = Tokenizer::new(src);

    let tags = Tags::parse(&tokenizer)?;

    let syntax_tree = SyntaxTree::new(tags)?;

    let resolved_tree = ResolvedTree::new(syntax_tree, ident_map)?;

    let relative_tree = RelativeTree::new(resolved_tree)?;

    println!("{:?}", relative_tree);

    Ok(())
}
