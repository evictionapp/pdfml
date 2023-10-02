use std::collections::HashMap;

use crate::lexical::attr::Attr;

use super::{error::ResolvedError, resolve_expr::resolve_expr};

pub fn resolve_attr<'a>(
    mut attr: Attr<'a>,
    ident_map: &mut HashMap<Box<str>, Box<str>>,
) -> Result<Attr<'a>, ResolvedError> {
    attr.expr = resolve_expr(attr.expr, ident_map)?;

    Ok(attr)
}
