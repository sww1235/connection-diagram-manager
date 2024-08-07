use std::fmt;

use super::super::file_types::svg::Svg as FileSvg;
use cdm_traits::partial_empty::PartialEmpty;

use cdm_macros::Empty;

/// Svg represents a full SVG image
#[derive(Debug, Default, Clone, PartialEq, Empty)]
pub struct Svg(pub String);

impl From<FileSvg> for Svg {
    fn from(file_svg: FileSvg) -> Self {
        Svg(file_svg.0)
    }
}

impl PartialEmpty for Svg {
    fn is_partial_empty(&self) -> bool {
        // only one field, may update this later
        self.0.is_empty()
    }
}

impl Svg {
    /// `new()` creates a new SVG
    #[must_use]
    pub fn new() -> Self {
        //TODO: actually have this be a blank SVG
        Self(String::new())
    }
}

impl fmt::Display for Svg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //TODO: fix this to be proper for SVG
        writeln!(f, "{}", self.0)?;
        Ok(())
    }
}
