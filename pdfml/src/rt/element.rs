use std::borrow::Cow;

use crate::{
    lexical::attributes::Attributes,
    relative::{
        error::RelativeError,
        relative_rect::{RelativeRect, TextContent},
    },
};

pub trait Element<'a> {
    fn relative_layout(attributes: &Attributes<'_>) -> Result<RelativeRect, RelativeError>;

    fn text_content(
        text_children: Vec<Cow<'a, str>>,
    ) -> Result<Option<TextContent<'a>>, RelativeError>;
}
