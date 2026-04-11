use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{
    datatypes::{
        library_types::LibraryData,
        svg::Svg,
        util_types::{Catalog, Dimension},
    },
    traits::FromFile,
};

//TODO: Make some of these fields enums
/// `EquipmentType` represents a type of equipment.
///
/// Anything from a rackmount piece of gear to an outlet or terminal block.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[expect(clippy::partial_pub_fields, reason = "contained_datafile_path is not part of public API")]
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
    /// datafile the struct instance was read in from.
    #[serde(skip)]
    pub(crate) contained_datafile_path: PathBuf,
}
impl FromFile for EquipmentType {
    #[inline]
    fn datafile(&self) -> PathBuf {
        self.contained_datafile_path.clone()
    }
    #[inline]
    fn set_datafile(&mut self, datafile_path: &Path) {
        self.contained_datafile_path = datafile_path.to_path_buf();
    }
}

// TODO: use custom SVG tags to store locations of connectors instead of x/y coordinates
/// `EquipFace` represents one physical face of equipment.
///
/// May have 2 faces for something like a patch panel, or 6 for a cube, or 1 for an unrolled
/// sphere, etc.
/// SVGs should be layed out for a horizontal orientation when defined.
/// instances can be rotated when defined in project.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EquipFace {
    /// visual representation of equipment face, without connectors.
    pub visual_representation: Option<Svg>,
    /// all connectors that are on this face of equipment.
    pub connectors: Option<BTreeMap<String, FaceConnector>>,
}

/// `FaceConnector` represents a connector on a face of an `EquipmentType`.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FaceConnector {
    //TODO: refcounted?
    /// Connector Type.
    connector_type: String,
    /// Signal direction.
    direction: Option<String>,
    /// location of connector from left of visual representation of face.
    x: u64,
    /// location of connector from bottom of visual representation of face.
    y: u64,
}

impl LibraryData for EquipmentType {}
