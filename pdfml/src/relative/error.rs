use thiserror::Error;

#[derive(Debug, Error)]
#[error("{attribute} for element {element_name}")]
pub struct AttributeError {
    pub attribute: Box<str>,
    pub element_name: &'static str,
}

#[derive(Debug, Error)]
pub enum RelativeError {
    #[error("expected attribute error = {0}")]
    ExpectedAttribute(AttributeError),
    #[error("unexpected attribute error = {0}")]
    UnexpectedAttribute(AttributeError),
    #[error("{0}")]
    Custom(Box<str>),
}
