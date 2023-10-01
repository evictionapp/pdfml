#[derive(Debug, Clone)]
pub struct Dimensions {
    pub position: Pair,
    pub widths: Pair,
    pub padding: Quad,
    pub border: Quad,
}

#[derive(Debug, Clone)]
pub struct Quad {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

#[derive(Debug, Clone)]
pub struct Pair {
    pub x: f32,
    pub y: f32,
}
