use retoken::Token;

use crate::{
    lexical::expr::Expr,
    relative::{
        error::{AttributeError, RelativeError},
        relative_rect::{DisplayType, Measurement, Pair, Quad, RelativeRect},
    },
};

use super::element::Element;

#[derive(Debug, Clone)]
pub struct Document {
    pub width: f32,
    pub heigth: f32,
}

impl<'a> Element<'a> for Document {
    fn relative_layout(
        attributes: &crate::lexical::attributes::Attributes<'_>,
    ) -> Result<RelativeRect, crate::relative::error::RelativeError> {
        let x = match &attributes
            .list
            .iter()
            .find(|attr| attr.ident.content() == "width")
            .ok_or(RelativeError::ExpectedAttribute(AttributeError {
                attribute: "width".into(),
                element_name: "document",
            }))?
            .expr
        {
            Expr::Quoted(quoted) => quoted,
            _ => return Err(RelativeError::Custom(r#"document expected width property to be a float followed by "mm", for example: "215.9mm"#.into())),
        };

        let x: f32 = match x.content().trim_end_matches("mm").parse() {
            Ok(float) => float,
            Err(_) => return Err(RelativeError::Custom(r#"document expected width property to be a float followed by "mm", for example: "215.9mm"#.into())),
        };

        let x = Measurement::Mm(x);

        let y = match &attributes
            .list
            .iter()
            .find(|attr| attr.ident.content() == "height")
            .ok_or(RelativeError::ExpectedAttribute(AttributeError {
                attribute: "height".into(),
                element_name: "document",
            }))?
            .expr
        {
            Expr::Quoted(quoted) => quoted,
            _ => return Err(RelativeError::Custom(r#"document expected height property to be a float followed by "mm", for example: "279.4mm"#.into())),
        };

        let y: f32 = match y.content().trim_end_matches("mm").parse() {
            Ok(float) => float,
            Err(_) => return Err(RelativeError::Custom(r#"document expected height property to be a float followed by "mm", for example: "279.4mm"#.into())),
        };

        let y = Measurement::Mm(y);

        Ok(RelativeRect {
            dimensions: Pair { x, y },
            padding: Quad::default(),
            border: Quad::default(),
            display_type: DisplayType::Col,
        })
    }

    fn text_content(
        text_children: Vec<std::borrow::Cow<'a, str>>,
    ) -> Result<Option<crate::relative::relative_rect::TextContent<'a>>, RelativeError> {
        match text_children.is_empty() {
            true => Ok(None),
            false => Err(RelativeError::Custom(
                "unexpected literal text children in document".into(),
            )),
        }
    }
}
