use retoken::Span;

use super::{
    error::{LexicalError, ParseError},
    parse::Parse,
    Alphabet, CloseAngle, Slash,
};

#[derive(Debug, Clone)]
pub enum ClosingType {
    Slash(Slash, CloseAngle),
    NoSlash(CloseAngle),
}

#[derive(Debug, Clone)]
pub struct Closing {
    pub closing_type: ClosingType,
    pub span: Span,
}

impl<'a> Parse<'a> for Closing {
    fn parse(tokenizer: &'a retoken::Tokenizer) -> Result<Self, LexicalError> {
        let start = tokenizer.cursor();
        let closing_type = match tokenizer.peek::<'a, Alphabet, Slash>() {
            true => ClosingType::Slash(
                tokenizer.token().map_err(Self::error)?,
                tokenizer.token().map_err(Self::error)?,
            ),
            false => ClosingType::NoSlash(tokenizer.token().map_err(Self::error)?),
        };
        let end = tokenizer.cursor();

        let span = Span { start, end };

        Ok(Self { closing_type, span })
    }

    fn error(error: retoken::Error<super::Alphabet>) -> LexicalError {
        LexicalError::Parse(ParseError {
            message: format!("Failed To Parse Closing Tag: {error}").into_boxed_str(),
            span: error.span(),
        })
    }
}

impl Closing {
    pub fn is_with_slash(&self) -> bool {
        matches!(self.closing_type, ClosingType::Slash(_, _))
    }
}
