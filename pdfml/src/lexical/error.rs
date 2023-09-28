use ariadne::{ColorGenerator, Fmt, Label, Report, ReportKind, Source};
use retoken::{SliceError, Span};
use thiserror::Error;

use super::Alphabet;

#[derive(Debug, Error)]
#[error("{message} @ {span:?}")]
pub struct ParseError {
    pub message: Box<str>,
    pub span: Span,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Token(retoken::Error<Alphabet>),
    #[error("{0}")]
    Parse(#[from] ParseError),
}

impl From<SliceError> for Error {
    fn from(value: SliceError) -> Self {
        Self::Token(retoken::Error::Slice(value))
    }
}

fn start_new_line(mut start: usize, src: &str) -> usize {
    let slice = match src.get(0..start) {
        Some(slice) => slice,
        None => return start,
    };

    for ch in slice.chars().rev() {
        if ch == '\n' {
            break;
        }
        start -= ch.len_utf8();
    }

    start
}

fn end_new_line(mut end: usize, src: &str) -> usize {
    let slice = match src.get(end..) {
        Some(slice) => slice,
        None => return end,
    };

    for ch in slice.chars().rev() {
        if ch == '\n' {
            break;
        }
        end += ch.len_utf8();
    }

    end
}

pub fn err_ariadne(err: &Error, src_id: &str, src: &str) -> std::io::Result<()> {
    let mut colors = ColorGenerator::new();

    let err_msg = match &err {
        Error::Parse(_) => "Parsing Error",
        Error::Token(_) => "Token Error",
    };

    let span = match &err {
        Error::Parse(p) => p.span.clone(),
        Error::Token(t) => t.span(),
    };

    let start = start_new_line(span.start, src);
    let end = end_new_line(span.end, src);

    Report::build(ReportKind::Error, src_id, 12)
        .with_message(err_msg)
        .with_label(
            Label::new((src_id, start..end))
                .with_message(err.to_string().fg(colors.next()))
                .with_color(colors.next()),
        )
        .finish()
        .print((src_id, Source::from(src)))
}
