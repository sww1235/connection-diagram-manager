use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::datatypes::{
    library_types::cable_type::{CableCore, CableLayer},
    unit_helper::cross_sectional_area::CrossSectionalArea,
    util_types::{Catalog, CrossSection, Dimension, LineStyle},
};

/// `CableType` is the source code representation of the on-disk file format for an in-memory
/// [`CableType`](crate::datatypes::library_types::cable_type::CableType).
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CableType {
    /// Catalog information.
    pub catalog: Option<Catalog>,
    /// Cable Type Code.
    ///
    /// SOOW, NM, USE, etc.
    pub cable_type_code: Option<String>,
    /// Cable cross sectional area.
    pub cross_sect_area: CrossSectionalArea,
    /// Cable cross section shape.
    ///
    /// Oval, circular, siamese.
    pub cross_section: CrossSection,
    /// Dimensions of cable.
    pub dimensions: Option<Dimension>,
    /// appearance in schematics.
    pub line_style: LineStyle,
    /// Map of cores in cable.
    ///
    /// Key of map is identifier of core within cable, and is unique within each cable.
    pub cores: BTreeMap<String, CableCore>,
    /// vector of exterior insulation/shielding layers.
    pub layers: Vec<CableLayer>,
}
