use std::path::PathBuf;

use uom::si::rational64::Area;

use cdm_macros::{Empty, Merge, PartialEmpty};
use cdm_traits::partial_empty::PartialEmpty;

use crate::datatypes::{
    color::Color,
    internal_types::svg::Svg,
    util_types::{Catalog, Dimension, LineStyle},
};

/// `PathwayType` represents a route for wires and cables to take from one
/// [`LocationType`](super::location_type::LocationType) to another.
///
/// Examples of Pathways include, conduit, cable tray, free air
#[derive(Debug, Default, PartialEq, Clone)]
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
    pub cross_sect_area: Area,
    /// Main material of pathway
    pub material: Option<String>,
    /// Primary color of pathway
    pub color: Option<Color>,
    /// Dimensions of pathway
    pub dimensions: Option<Dimension>,
    /// appearance in schematics
    pub line_style: Option<LineStyle>,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}
impl PathwayType {
    /// Creates an empty instance of `PathwayType`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
