use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uom::si::rational64::Length;

use crate::datatypes::{
    color::Color,
    internal_types::svg::Svg,
    util_types::{Catalog, Dimension},
};

use cdm_macros::{Empty, Merge, PartialEmpty};
use cdm_traits::partial_empty::PartialEmpty;

//TODO: create physical location stuff
/// `EnclosureType` represents a type/model of location.
///
/// Examples of `EnclosureType` include junction boxes, racks, panels, etc.
/// It does not include places these are located.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnclosureType {
    /// Catalog information
    pub catalog: Option<Catalog>,
    /// Dimensional information of enclosure
    pub dimensions: Option<Dimension>,
    /// Main material of Enclosure Type
    pub material: Option<String>,
    /// Usable Width
    pub usable_width: Length,
    /// Usable Height
    pub usable_height: Length,
    /// Usable Depth
    pub usable_depth: Option<Length>,
    /// Visual representation of Enclosure
    pub visual_representation: Option<Svg>,
    /// Primary color of enclosure
    pub color: Option<Color>,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}

impl EnclosureType {
    /// Creates an empty instance of `EnclosureType`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
