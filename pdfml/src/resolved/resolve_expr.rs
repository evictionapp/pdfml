use std::collections::HashMap;

use retoken::Token;

use crate::lexical::expr::Expr;

use super::error::{ExpectedIdentError, ResolvedError};

pub fn resolve_expr<'a>(
    expr: Expr<'a>,
    ident_map: &mut HashMap<Box<str>, Box<str>>,
) -> Result<Expr<'a>, ResolvedError> {
    let ok = match expr {
        Expr::Quoted(quoted) => Expr::Quoted(quoted),
        Expr::Resolved(resolved) => Expr::Resolved(resolved),
        Expr::Bracketed(bracketed) => {
            let content = bracketed.ident.content();
            let value = match ident_map.remove(content) {
                Some(value) => value,
                None => {
                    return Err(ResolvedError::ExpectedIdent(ExpectedIdentError {
                        ident: content.into(),
                        span: bracketed.ident.span(),
                    }))
                }
            };

            Expr::Resolved(value)
        }
    };

    Ok(ok)
}
