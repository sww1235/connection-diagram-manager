use super::{connector_type::ConnectorType, svg::Svg};

use cdm_macros::{Empty, Merge, PartialEmpty};

use std::cell::RefCell;
use std::fmt;
use std::path::PathBuf;
use std::rc::Rc;

//TODO: Make some of these fields enums
/// `EquipmentType` represents a type of equipment
///
/// Anything from a rackmount piece of gear to an outlet or terminal block
#[derive(Debug, Default, PartialEq, Merge, PartialEmpty, Empty)]
pub struct EquipmentType {
    //TODO: add dimensions here
    /// Internal ID of `EquipmentType`
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
    pub mount_type: Option<String>,
    /// Equipment Type (audio, video, mix, lighting, networking, patch panel, power)
    pub equip_type: Option<String>,
    /// faces represents a visual representation of each face of a piece of equipment
    pub faces: Option<Vec<EquipFace>>,
    /// visual representation of the equipment
    // TODO: figure out what angle to standardize on, or
    // just rely on the face vis_rep
    pub visual_rep: Svg,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}

/// `EquipFace` represents one physical face of equipment.
///
/// May have 2 faces for something like a patch panel, or 6 for a cube, or 1 for an unrolled
/// sphere, etc.
#[derive(Debug, Default, PartialEq, Clone)]
pub struct EquipFace {
    /// Name/ID of equipment face
    pub name: String,
    /// visual representation of equipment face, without connectors
    pub vis_rep: Option<Svg>,
    /// all connectors that are on this face of equipment
    pub connectors: Option<Vec<EquipConnector>>,
}

//TODO: Make some of these fields enums
/// `EquipmentConnector` represents an instance of a [`ConnectorType`](super::connector_type::ConnectorType) in
/// a `EquipmentType`
#[derive(Debug, Default, PartialEq, Clone)]
pub struct EquipConnector {
    /// Internal ID of `EquipmentConnector`
    /// ConnectorType
    pub connector_type: Rc<RefCell<ConnectorType>>,
    /// electrical direction, used for basic rule mapping, (input, output, power input, power
    /// output, bidirectiona, passive)
    pub direction: Option<String>,
    /// location of connector on face from left of visrep. Origin is bottom left
    pub x: u64, //TODO: Units?
    /// location of connector on face from bottom of visrep. Origin is bottom left
    pub y: u64,
}
impl EquipmentType {
    /// Creates an empty instance of `EquipmentType`
    #[allow(clippy::arithmetic_side_effects)]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl fmt::Display for EquipConnector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Equipment Connector:")?;
        writeln!(f, "Connector: {}", &self.connector_type.borrow())?;
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
