use super::svg::Svg;
use serde::{Deserialize, Serialize};

use std::fmt;
//TODO: Make some of these fields enums
/// EquipmentType represents a type of equipment.
///
/// Anything from a rackmount piece of gear to an outlet or terminal block. This represents
/// something that is off the shelf, or at least self contained and does not have internal
/// connections that needs to be known to this tool.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct EquipmentType {
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
    pub vis_rep: Option<Svg>,
    /// all connectors on face
    pub connectors: Option<Vec<EquipConnector>>,
}
/// `EquipmentConnector` represents an instance of a [`ConnectorType`](super::connector_type::ConnectorType) in
/// a `EquipmentType`
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct EquipConnector {
    /// ConnectorType
    pub connector_type: String,
    /// electrical direction, used for basic rule mapping, (input, output, power input, power
    /// output, bidirectiona, passive)
    pub direction: Option<String>,
    /// location of connector on face from left of visrep. Origin is bottom left
    pub x: u64,
    /// location of connector on face from bottom of visrep. Origin is bottom left
    pub y: u64,
}

impl fmt::Display for EquipConnector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Equipment Connector:")?;
        writeln!(f, "Connector: {}", &self.connector_type)?;
        if let Some(direction) = &self.direction {
            writeln!(f, "Direction: {direction}")?;
        }
        writeln!(f, "X coordinate: {}", self.x)?;
        writeln!(f, "Y coordinate: {}", self.y)?;
        Ok(())
    }
}

impl fmt::Display for EquipmentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Equipment Type:")?;
        if let Some(manufacturer) = &self.manufacturer {
            write!(f, "Manufacturer: {manufacturer}")?;
        }
        if let Some(model) = &self.model {
            write!(f, "Model: {model}")?;
        }
        if let Some(part_number) = &self.part_number {
            write!(f, "Part Number: {part_number}")?;
        }
        if let Some(manufacturer_part_number) = &self.manufacturer_part_number {
            write!(f, "Manufacturer Part Number: {manufacturer_part_number}")?;
        }
        if let Some(supplier) = &self.supplier {
            write!(f, "Supplier: {supplier}")?;
        }
        if let Some(supplier_part_number) = &self.supplier_part_number {
            write!(f, "Supplier Part Number: {supplier_part_number}")?;
        }
        if let Some(description) = &self.description {
            write!(f, "Description: {description}")?;
        }
        if let Some(mount_type) = &self.mount_type {
            write!(f, "Mount Type: {mount_type}")?;
        }
        if let Some(equip_type) = &self.equip_type {
            write!(f, "Equipment Type: {equip_type}")?;
        }
        //TODO: implement loops over faces and connectors
        //TODO: implement svg validation rules here
        Ok(())
    }
}
