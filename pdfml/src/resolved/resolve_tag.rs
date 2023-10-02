use std::collections::HashMap;

use crate::syntax::tag_node::TagNode;

use super::{error::ResolvedError, resolve_attr::resolve_attr};

pub fn resolve_tag<'a>(
    mut tag: TagNode<'a>,
    ident_map: &mut HashMap<Box<str>, Box<str>>,
) -> Result<TagNode<'a>, ResolvedError> {
    tag.attributes.list = tag
        .attributes
        .list
        .into_iter()
        .map(|attr| resolve_attr(attr, ident_map))
        .collect::<Result<_, _>>()?;

    Ok(tag)
}
