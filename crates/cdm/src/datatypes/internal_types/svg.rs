use std::fmt;

use super::super::file_types::svg::Svg as FileSvg;
use cdm_traits::{empty::Empty, partial_empty::PartialEmpty};

/// Svg represents a full SVG image
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Svg(pub String);

impl From<FileSvg> for Svg {
    fn from(file_svg: FileSvg) -> Self {
        Svg(file_svg.0)
    }
}
//impl Mergable for Svg {
//    fn merge_prompt(
//        &mut self,
//        _other: &Self,
//        _prompt_fn: fn(HashMap<String, [String; 2]>) -> HashMap<String, bool>,
//    ) {
//        todo!();
//    }
//}

impl Empty for Svg {
    fn is_empty(&self) -> bool {
        self == &Self::new()
    }
}

impl PartialEmpty for Svg {
    fn is_partial_empty(&self) -> bool {
        // only one field, may update this later
        self.is_empty()
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
