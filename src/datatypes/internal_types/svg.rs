use super::super::file_types::svg::Svg as FileSvg;

/// Svg represents a full SVG image
#[derive(Debug, Default, Clone)]
pub struct Svg(pub String);

impl From<FileSvg> for Svg {
    fn from(file_svg: FileSvg) -> Self {
        Svg(file_svg.0)
    }
}

impl Svg {
    /// `new()` creates a new SVG
    pub fn new() -> Self {
        //TODO: actually have this be a blank SVG
        Svg(String::new())
    }
}
