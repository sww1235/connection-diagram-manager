use super::svg::Svg;
use serde::{Deserialize, Serialize};

/// `EquipmentType` represents a type of equipment.
///
/// Anything from a rackmount piece of gear to an outlet or terminal block. This represents
/// something that is off the shelf, or at least self contained and does not have internal
/// connections that needs to be known to this tool.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct EquipmentType {
    /// Manufacturer of `Equipment`
    pub manufacturer: Option<String>,
    /// Model of Equipment
    pub model: Option<String>,
    /// Part Number of `Equipment`
    pub part_number: Option<String>,
    /// Manufacturer's Part Number
    pub manufacturer_part_number: Option<String>,
    /// Supplier of `Equipment`
    pub supplier: Option<String>,
    /// Supplier's Part Number
    pub supplier_part_number: Option<String>,
    /// Optional text description
    pub description: Option<String>,
    /// List of mounting options for equipment
    pub mount_type: Option<String>,
    /// Equipment Type (audio, video, mix, lighting, networking, patch panel, power)
    pub equip_type: Option<String>,
    /// `faces` contains representations of each face of the equipment
    ///
    /// May have 2 faces for something like a patch panel, or 6 for a cube, or 1 for an unrolled
    /// sphere, etc.
    pub faces: Option<Vec<EquipFace>>,
    /// visual representation of the equipment
    // TODO: figure out what angle to standardize on, or
    // just rely on the face vis_rep
    pub visual_rep: Svg,
}

/// `EquipFace` represents one physical face of equipment.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct EquipFace {
    /// Name of face
    pub name: String,
    /// Visual representation of face in SVG format, without connectors
    pub visual_rep: Svg,
    /// all connectors on face
    pub connectors: Option<Vec<EquipConnector>>,
}
/// `EquipmentConnector` represents an instance of a [`ConnectorType`](super::connector_type::ConnectorType) in
/// a `EquipmentType`
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct EquipConnector {
    /// `ConnectorType`
    pub connector_type: String,
    /// electrical direction, used for basic rule mapping, (input, output, power input, power
    /// output, bidirectiona, passive)
    pub direction: Option<String>,
    /// location of connector on face from left of visrep. Origin is bottom left
    pub x: f64,
    /// location of connector on face from bottom of visrep. Origin is bottom left
    pub y: f64,
}
