use serde::{Deserialize, Serialize};

use crate::datatypes::{
    color::Color,
    svg::Svg,
    unit_helper::length::Length,
    util_types::{Catalog, Dimension},
};

/// `CableType` is the source code representation of the on-disk file format for an in-memory
/// [`CableType`](crate::datatypes::library_types::cable_type::CableType).
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EnclosureType {
    /// Catalog information.
    pub catalog: Option<Catalog>,
    /// Dimensional information of enclosure.
    pub dimensions: Dimension,
    /// Main material of `EnclosureType`.
    pub material: Option<String>,
    /// Usable Width.
    pub usable_width: Length,
    /// Usable Height.
    pub usable_height: Length,
    /// Usable Depth.
    pub usable_depth: Option<Length>,
    /// Other rating information for enclosure.
    pub rating: Option<String>,
    /// Visual representation of Enclosure.
    pub visual_representation: Option<Svg>,
    /// Primary color of enclosure.
    pub color: Option<Color>,
}
