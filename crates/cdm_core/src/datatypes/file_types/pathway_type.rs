use serde::{Deserialize, Serialize};

use crate::datatypes::{
    color::Color,
    svg::Svg,
    unit_helper::cross_sectional_area::CrossSectionalArea,
    util_types::{Catalog, Dimension, LineStyle},
};

/// `PathwayType` is the source code representation of the on-disk file format for an in-memory
/// [`PathwayType`](crate::datatypes::library_types::pathway_type::PathwayType).
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
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
}
