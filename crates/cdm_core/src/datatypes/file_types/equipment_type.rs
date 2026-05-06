use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::datatypes::{
    library_types::equipment_type::EquipFace,
    svg::Svg,
    util_types::{Catalog, Dimension},
};

/// `EquipmentType` is the source code representation of the on-disk file format for an in-memory
/// [`EquipmentType`](crate::datatypes::library_types::equipment_type::EquipmentType).
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EquipmentType {
    /// Catalog information.
    pub catalog: Option<Catalog>,
    /// Dimensional information of equipment.
    pub dimensions: Option<Dimension>,
    /// List of mounting options for equipment.
    pub mount_types: Vec<String>,
    /// Equipment Type (audio, video, mix, lighting, networking, patch panel, power).
    pub category: Option<String>,
    /// Equipment supertype: Relay, PLC, Motor, Relay, Circuit breaker, etc.
    pub supertype: Option<String>,
    /// Component Designator.
    pub component_designator: Option<String>,
    /// Rating of equipment. Not parsed.
    pub rating: Option<String>,
    /// Vector of schematic symbols that can represent this equipment.
    /// values must be the id of the `symbol_type`.
    pub schematic_symbols: Vec<String>,
    /// visual representation of the equipment.
    // TODO: figure out what angle to standardize on, or
    // just rely on the face vis_rep
    // TODO: create associated method to return correct face here
    pub visual_representation: Option<Svg>,
    /// faces represents a visual representation of each face of a piece of equipment.
    pub faces: Option<BTreeMap<String, EquipFace>>,
}
