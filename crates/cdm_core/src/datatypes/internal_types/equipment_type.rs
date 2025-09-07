use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use uom::si::rational64::Length;

use cdm_macros::{Empty, Merge, PartialEmpty};
use cdm_traits::partial_empty::PartialEmpty;

use super::{connector::Connector, svg::Svg};

//TODO: Make some of these fields enums
/// `EquipmentType` represents a type of equipment
///
/// Anything from a rackmount piece of gear to an outlet or terminal block
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EquipmentType {
    //TODO: add dimensions here
    /// Internal ID of Equipment Type
    pub id: String,
    /// Manufacturer of Equipment
    pub manufacturer: Option<String>,
    /// Model of Equipment
    pub model: Option<String>,
    /// Part Number of Equipment
    pub part_number: Option<String>,
    /// Manufacturer's Part Number
    pub manufacturer_part_number: Option<String>,
    /// Supplier of Equipment
    pub supplier: Option<String>,
    /// Supplier's Part Number
    pub supplier_part_number: Option<String>,
    /// Optional text description
    pub description: Option<String>,
    /// List of mounting options for equipment
    pub mount_types: Vec<String>,
    /// Equipment Type (audio, video, mix, lighting, networking, patch panel, power)
    pub equip_type: Option<String>,
    /// faces represents a visual representation of each face of a piece of equipment
    pub faces: Option<HashMap<String, EquipFace>>,
    /// visual representation of the equipment
    // TODO: figure out what angle to standardize on, or
    // just rely on the face vis_rep
    // TODO: create associated method to return correct face here
    pub visual_rep: Svg,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}

/// `EquipFace` represents one physical face of equipment.
///
/// May have 2 faces for something like a patch panel, or 6 for a cube, or 1 for an unrolled
/// sphere, etc.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EquipFace {
    /// visual representation of equipment face, without connectors
    pub visual_rep: Svg,
    /// all connectors that are on this face of equipment
    pub connectors: Option<Vec<ConnectorJoin>>,
}

//TODO: Make some of these fields enums
/// `ConnectorJoin` rep in
/// a `EquipmentType`
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ConnectorJoin {
    /// `Connector`
    pub connector: Rc<RefCell<Connector>>,
    /// electrical direction, used for basic rule mapping, (input, output, power input, power
    /// output, bidirectiona, passive)
    pub direction: Option<String>,
    /// location of connector on face from left of visrep. Origin is bottom left
    pub x: Length,
    /// location of connector on face from bottom of visrep. Origin is bottom left
    pub y: Length,
}
impl EquipmentType {
    /// Creates an empty instance of `EquipmentType`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns representative svg representation of `EquipmentType`
    #[must_use]
    pub fn visual_rep(&self) -> Svg {
        match &self.faces {
            Some(faces) => faces["Front"].visual_rep.clone(),
            None => self.visual_rep.clone(),
        }
    }
}
