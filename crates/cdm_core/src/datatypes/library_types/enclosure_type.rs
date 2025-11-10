use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{
    datatypes::{
        color::Color,
        svg::Svg,
        unit_helper::Length,
        util_types::{Catalog, Dimension},
    },
    traits::FromFile,
};

//TODO: create physical location stuff
/// `EnclosureType` represents a type/model of location.
///
/// Examples of `EnclosureType` include junction boxes, racks, panels, etc.
/// It does not include places these are located.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnclosureType {
    /// Catalog information
    pub catalog: Option<Catalog>,
    /// Dimensional information of enclosure
    pub dimensions: Dimension,
    /// Main material of Enclosure Type
    pub material: Option<String>,
    /// Usable Width
    pub usable_width: Length,
    /// Usable Height
    pub usable_height: Length,
    /// Usable Depth
    pub usable_depth: Option<Length>,
    /// Other rating information for enclosure
    pub rating: Option<String>,
    /// Visual representation of Enclosure
    pub visual_representation: Option<Svg>,
    /// Primary color of enclosure
    pub color: Option<Color>,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}

impl FromFile for EnclosureType {
    fn datafile(&self) -> PathBuf {
        self.contained_datafile_path.clone()
    }
}
