use std::borrow::Cow;

#[derive(Debug, Clone)]
pub enum DisplayType {
    Row,
    Col,
}

#[derive(Debug, Clone)]
pub struct TextContent<'a> {
    pub text: Cow<'a, str>,
    pub font_size: f32,
    pub font_family: (),
}

#[derive(Debug, Clone)]
pub enum Measurement {
    Mm(f32),
    Pct(f32),
}

#[derive(Debug, Clone)]
pub struct Pair {
    pub x: Measurement,
    pub y: Measurement,
}

#[derive(Debug, Clone, Default)]
pub struct Quad {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

#[derive(Debug, Clone)]
pub struct RelativeRect {
    pub dimensions: Pair,
    pub padding: Quad,
    pub border: Quad,
    pub display_type: DisplayType,
}
