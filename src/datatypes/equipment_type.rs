use super::{equipment_connector, svg};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use std::fmt;
//TODO: Make some of these fields enums
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct EquipmentType {
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub part_number: Option<String>,
    pub manufacturer_part_number: Option<String>,
    pub supplier: Option<String>,
    pub supplier_part_number: Option<String>,
    pub description: Option<String>,
    pub mount_type: Option<Vec<String>>,
    pub equip_type: Option<String>,
    pub faces: Option<HashMap<String, svg::Svg>>,
    pub visual_rep: Option<svg::Svg>,
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
