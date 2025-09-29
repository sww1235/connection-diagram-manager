use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::datatypes::{
    library_types::connector_type::ConnectorType,
    svg::Svg,
    util_types::{Catalog, Dimension},
};

//TODO: Make some of these fields enums
/// `EquipmentType` represents a type of equipment
///
/// Anything from a rackmount piece of gear to an outlet or terminal block
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EquipmentType {
    /// Catalog information
    pub catalog: Option<Catalog>,
    /// Dimensional information of equipment
    pub dimensions: Option<Dimension>,
    /// List of mounting options for equipment
    pub mount_types: Vec<String>,
    /// Equipment Type (audio, video, mix, lighting, networking, patch panel, power)
    pub category: Option<String>,
    /// Equipment supertype: Relay, PLC, Motor, Relay, Circuit breaker, etc.
    pub supertype: Option<String>,
    /// Component Designator
    pub component_designator: Option<String>,
    /// Rating of equipment. Not parsed
    pub rating: Option<String>,
    /// Vector of schematic symbols that can represent this equipment.
    /// values must be the id of the symbol_type
    pub schematic_symbols: Option<Vec<String>>,
    /// visual representation of the equipment
    // TODO: figure out what angle to standardize on, or
    // just rely on the face vis_rep
    // TODO: create associated method to return correct face here
    pub visual_representation: Option<Svg>,
    /// faces represents a visual representation of each face of a piece of equipment
    pub faces: Option<HashMap<String, EquipFace>>,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}

// TODO: use custom SVG tags to store locations of connectors instead of x/y coordinates
/// `EquipFace` represents one physical face of equipment.
///
/// May have 2 faces for something like a patch panel, or 6 for a cube, or 1 for an unrolled
/// sphere, etc.
/// SVGs should be layed out for a horizontal orientation when defined.
/// instances can be rotated when defined in project.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EquipFace {
    /// visual representation of equipment face, without connectors
    pub visual_representation: Option<Svg>,
    /// all connectors that are on this face of equipment
    pub connectors: Option<HashMap<String, FaceConnector>>,
}

/// `FaceConnector` represents
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FaceConnector {
    //TODO: refcounted?
    /// Connector Type
    connector_type: ConnectorType,
    direction: Option<String>,
    x: u64,
    y: u64,
}

impl EquipmentType {
    /// Returns representative svg representation of `EquipmentType`
    #[must_use]
    pub fn visual_rep(&self) -> Svg {
        match &self.faces {
            Some(faces) => faces["Front"]
                .visual_representation
                .clone()
                .unwrap_or_default()
                .clone(),
            None => self
                .visual_representation
                .clone()
                .unwrap_or_default()
                .clone(),
        }
    }
}
