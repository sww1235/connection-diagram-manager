use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{
    datatypes::{
        color::Color,
        svg::Svg,
        unit_helper::CrossSectionalArea,
        util_types::{Catalog, Dimension, LineStyle},
    },
    traits::FromFile,
};

/// `PathwayType` represents a route for wires and cables to take from one
/// [`LocationType`](super::location_type::LocationType) to another.
///
/// Examples of Pathways include, conduit, cable tray, free air
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PathwayType {
    /// Catalog information
    pub catalog: Option<Catalog>,
    /// generic type of pathway: (conduit, cable tray, etc)
    pub supertype: Option<String>,
    /// actual size of pathway
    pub size: Option<String>,
    /// Trade Size of pathway
    pub trade_size: Option<String>,
    /// Visual representation of pathway
    /// used to display a representation of the pathway on panel diagrams
    /// mainly used for things like panduit or wireway mounted to panel directly
    pub visual_representation: Option<Svg>,
    /// Inner cross sectional area of pathway
    pub cross_sect_area: CrossSectionalArea,
    /// Main material of pathway
    pub material: Option<String>,
    /// Primary color of pathway
    pub color: Option<Color>,
    /// Material properties/rating. Not parsed.
    ///
    /// Voltage/Temperature/Flammability/etc.
    pub rating: Option<String>,
    /// Dimensions of pathway
    pub dimensions: Option<Dimension>,
    /// appearance in schematics
    pub line_style: Option<LineStyle>,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}
impl FromFile for PathwayType {
    fn datafile(&self) -> PathBuf {
        self.contained_datafile_path.clone()
    }
}
