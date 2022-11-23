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
        if let Some(foo) = &self.manufacturer {
            write!(f, "Manufacturer: {}", foo)?;
        }
        if let Some(foo) = &self.model {
            write!(f, "Model: {}", foo)?;
        }
        if let Some(foo) = &self.part_number {
            write!(f, "Part Number: {}", foo)?;
        }
        if let Some(foo) = &self.manufacturer_part_number {
            write!(f, "Manufacturer Part Number: {}", foo)?;
        }
        if let Some(foo) = &self.supplier {
            write!(f, "Supplier: {}", foo)?;
        }
        if let Some(foo) = &self.supplier_part_number {
            write!(f, "Supplier Part Number: {}", foo)?;
        }
        if let Some(foo) = &self.description {
            write!(f, "Description: {}", foo)?;
        }
        if let Some(foo) = &self.mount_type {
            if foo.len() == 1 {
                write!(f, "Mount Type: {}", foo[0])?;
            } else {
                write!(f, "Mount Types:")?;
                for foo_item in foo.iter() {
                    write!(f, "\t- {}", foo_item)?;
                }
            }
        }
        if let Some(foo) = &self.equip_type {
            write!(f, "Equipment Type: {}", foo)?;
        }
        //TODO: implement loops over faces and connectors
        //TODO: implement svg validation rules here
        Ok(())
    }
}
