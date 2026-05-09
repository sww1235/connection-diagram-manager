use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    datatypes::{
        color::Color,
        file_types,
        library_types::LibraryData,
        svg::Svg,
        unit_helper::cross_sectional_area::CrossSectionalArea,
        util_types::{Catalog, Dimension, LineStyle},
    },
    traits::FromFile,
};

/// `PathwayType` represents a route for wires and cables to take from one
/// [`LocationType`](super::location_type::LocationType) to another.
///
/// Examples of Pathways include, conduit, cable tray, free air.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[expect(clippy::partial_pub_fields, reason = "contained_datafile_path is not part of public API")]
pub struct PathwayType {
    /// Catalog information.
    pub catalog: Option<Catalog>,
    /// generic type of pathway: (conduit, cable tray, etc).
    pub supertype: Option<String>,
    /// actual size of pathway.
    pub size: Option<String>,
    /// Trade Size of pathway.
    pub trade_size: Option<String>,
    /// Visual representation of pathway.
    ///
    /// Used to display a representation of the pathway on panel diagrams.
    /// Mainly used for things like Panduit or wireway mounted to panel directly.
    pub visual_representation: Option<Svg>,
    /// Inner cross sectional area of pathway.
    pub cross_sect_area: Option<CrossSectionalArea>,
    /// Main material of pathway.
    pub material: Option<String>,
    /// Primary color of pathway.
    pub color: Option<Color>,
    /// Material properties/rating. Not parsed.
    ///
    /// Voltage/Temperature/Flammability/etc.
    pub rating: Option<String>,
    /// Dimensions of pathway.
    pub dimensions: Option<Dimension>,
    /// appearance in schematics.
    pub line_style: Option<LineStyle>,
    /// datafile the struct instance was read in from.
    #[serde(skip)]
    pub(crate) contained_datafile_path: PathBuf,
}

impl From<file_types::pathway_type::PathwayType> for PathwayType {
    #[inline]
    fn from(value: file_types::pathway_type::PathwayType) -> Self {
        Self {
            catalog: value.catalog,
            supertype: value.supertype,
            size: value.size,
            trade_size: value.trade_size,
            visual_representation: value.visual_representation,
            cross_sect_area: value.cross_sect_area,
            material: value.material,
            color: value.color,
            rating: value.rating,
            dimensions: value.dimensions,
            line_style: value.line_style,
            contained_datafile_path: PathBuf::new(),
        }
    }
}

impl FromFile for PathwayType {
    #[inline]
    fn datafile(&self) -> PathBuf {
        self.contained_datafile_path.clone()
    }
    #[inline]
    fn set_datafile(&mut self, datafile_path: &Path) {
        self.contained_datafile_path = datafile_path.to_path_buf();
    }
}
impl LibraryData for PathwayType {}
