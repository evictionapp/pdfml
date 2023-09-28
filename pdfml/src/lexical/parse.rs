use retoken::Tokenizer;

use super::{error::Error, Alphabet};

pub trait Parse<'a>: 'a + Sized {
    fn parse(tokenizer: &'a Tokenizer) -> Result<Self, Error>;

    fn error(error: retoken::Error<Alphabet>) -> Error;
}
