use super::{equipment_connector::EquipmentConnector, svg::Svg};
use std::collections::HashMap;

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
//TODO: Make some of these fields enums
/// EquipmentType represents a type of equipment
///
/// Anything from a rackmount piece of gear to an outlet or terminal block
#[derive(Debug, Default)]
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
    // TODO maybe make this just one string and require different equipmentType records per
    // mounting type
    /// List of mounting options for equipment
    pub mount_type: Option<Vec<String>>,
    /// Equipment Type (audio, video, mix, lighting, networking, patch panel, power)
    pub equip_type: Option<String>,
    /// TODO: create a separate face type
    /// faces represents a visual representation of each face of a piece of equipment
    pub faces: Option<HashMap<String, svg::Svg>>,
    /// visual representation of the equipment
    // TODO: figure out what angle to standardize on, or
    // just rely on the face vis_rep
    pub visual_rep: Option<svg::Svg>,
    /// list of connectors in equipment. Contains position data for the connectors
    // TODO: Merge into face type
    pub connectors: Option<HashMap<String, equipment_connector::EquipmentConnector>>,
}
impl fmt::Display for EquipmentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Equipment Type:")?;
        if let Some(manufacturer) = &self.manufacturer {
            write!(f, "Manufacturer: {}", manufacturer)?;
        }
        if let Some(model) = &self.model {
            write!(f, "Model: {}", model)?;
        }
        if let Some(part_number) = &self.part_number {
            write!(f, "Part Number: {}", part_number)?;
        }
        if let Some(manufacturer_part_number) = &self.manufacturer_part_number {
            write!(f, "Manufacturer Part Number: {}", manufacturer_part_number)?;
        }
        if let Some(supplier) = &self.supplier {
            write!(f, "Supplier: {}", supplier)?;
        }
        if let Some(supplier_part_number) = &self.supplier_part_number {
            write!(f, "Supplier Part Number: {}", supplier_part_number)?;
        }
        if let Some(description) = &self.description {
            write!(f, "Description: {}", description)?;
        }
        if let Some(mount_type) = &self.mount_type {
            if mount_type.len() == 1 {
                write!(f, "Mount Type: {}", mount_type[0])?;
            } else {
                write!(f, "Mount Types:")?;
                for mount_type_item in mount_type.iter() {
                    write!(f, "\t- {}", mount_type_item)?;
                }
            }
        }
        if let Some(equip_type) = &self.equip_type {
            write!(f, "Equipment Type: {}", equip_type)?;
        }
        //TODO: implement loops over faces and connectors
        //TODO: implement svg validation rules here
        Ok(())
    }
}
