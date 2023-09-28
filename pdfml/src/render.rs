use retoken::Tokenizer;

use crate::lexical::{error::err_ariadne, parse::Parse, tags::Tags};

pub fn render(src: &str) {
    let tokenzier = Tokenizer::new(src);

    let tags = Tags::parse(&tokenzier);

    match tags {
        Ok(ok) => {
            println!("{:?}", ok);
        }
        Err(err) => {
            let _ = err_ariadne(&err, "foo.pdfml", src);
        }
    }
}
