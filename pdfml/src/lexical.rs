use retoken::{relex, Span};

pub mod arg;
pub mod arguments;
pub mod attr;
pub mod attributes;
pub mod bracketed_expr;
pub mod call_expr;
pub mod closing;
pub mod error;
pub mod expr;
pub mod literal;
pub mod markup;
pub mod markup_close;
pub mod markup_open;
pub mod markup_tag;
pub mod parse;
pub mod tag;
pub mod tags;
pub mod unquoted_literal;

relex! {
    pub alphabet Alphabet
    pub grammar Grammar

    pub skip Skip r"\s+"

    pub char OpenAngle '<'
    pub char CloseAngle '>'
    pub char OpenBracket '{'
    pub char CloseBracket '}'
    pub char OpenParen '('
    pub char CloseParen ')'
    pub char Slash '/'
    pub char Equal '='
    pub char Comma ','

    pub re Ident "[a-zA-Z][a-zA-Z0-9_]*"

    pub re Integer "[0-9]+"
    pub re Float r"[0-9]+\.[0-9]+"

    pub borrowed Quoted
}

#[derive(Debug, Clone)]
pub struct Quoted<'a> {
    content: &'a str,
    span: Span,
}

impl<'a> retoken::Token<'a, Alphabet> for Quoted<'a> {
    fn content(&'a self) -> &'a str {
        self.content
    }

    fn span(&'a self) -> Span {
        self.span.clone()
    }

    fn peek(tokenizer: &'a retoken::Tokenizer) -> bool {
        tokenizer.peek_char('"')
    }

    fn token(tokenizer: &'a retoken::Tokenizer) -> retoken::Result<Self, Alphabet> {
        if !tokenizer.peek_char('"') {
            return Err(retoken::Error::expected_token(
                tokenizer.cursor(),
                Alphabet::Quoted,
            ));
        }

        let (slice, start) = tokenizer.slice_with_cursor()?;

        let mut len = 0;
        let mut prev_char = '\\';
        for ch in slice.chars() {
            len += ch.len_utf8();
            if ch == '"' && prev_char != '\\' && len > 0 {
                break;
            }
            prev_char = ch;
        }

        let span = Span {
            start,
            end: start + len,
        };
        let content = tokenizer.slice_with_span(span.clone())?;
        if !content.ends_with('"') {
            return Err(retoken::Error::expected_token(
                start + len,
                Alphabet::Quoted,
            ));
        }

        tokenizer.advance(len);

        Ok(Self { content, span })
    }
}
