use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::datatypes::{
    library_types::cable_type::CableLayer,
    unit_helper::{cross_sectional_area::CrossSectionalArea, nominal_core_size::NominalCoreSize},
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
    pub type_code: Option<String>,
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

/// `CableCore` represents an individual conductor, strength member or optical fiber in a cable.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CableCore {
    /// ID of `CableType` that this core is made of.
    pub type_id: String,
    /// `LineStyle` of `CableType`. If `None`, then it will inherit from the parent `CableType`.
    pub line_style: Option<LineStyle>,
    /// The material the core is made out of.
    pub material: Option<String>,
    /// Cross sectional area.
    pub cross_sect_area: Option<CrossSectionalArea>,
    /// Nominal size of core.
    pub nominal_size: Option<NominalCoreSize>,
    /// If `CableCore` is stranded.
    pub stranded: Option<bool>,
    /// How many strands is conductor made of.
    pub num_strands: Option<u64>,
    /// Cross sectional area of individual strand.
    pub strand_cross_sect_area: Option<CrossSectionalArea>,
}
