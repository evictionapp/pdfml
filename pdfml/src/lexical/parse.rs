use retoken::Tokenizer;

use super::{error::LexicalError, Alphabet};

pub trait Parse<'a>: 'a + Sized {
    fn parse(tokenizer: &'a Tokenizer) -> Result<Self, LexicalError>;

    fn error(error: retoken::Error<Alphabet>) -> LexicalError;
}
