use std::collections::HashMap;

use super::super::file_types::svg::Svg as FileSvg;
use super::{Empty, Mergable, PartialEmpty};

/// Svg represents a full SVG image
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Svg(pub String);

impl From<FileSvg> for Svg {
    fn from(file_svg: FileSvg) -> Self {
        Svg(file_svg.0)
    }
}
impl Mergable for Svg {
    fn merge_prompt(
        &mut self,
        other: &Self,
        prompt_fn: fn(HashMap<String, [String; 2]>) -> HashMap<String, u8>,
    ) -> Self {
        todo!();
    }
}

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
    pub fn new() -> Self {
        //TODO: actually have this be a blank SVG
        Self(String::new())
    }
}
