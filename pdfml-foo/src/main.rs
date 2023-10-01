use std::collections::HashMap;

use pdfml::render::render;

fn main() {
    let foo = include_str!("../../foo.pdfml");

    let mut ident_map = HashMap::new();

    ident_map.insert("case_number".into(), "JP01-23-E00123".into());

    if let Err(err) = render(foo, ident_map) {
        eprintln!("{:?}", err);
    }
}
