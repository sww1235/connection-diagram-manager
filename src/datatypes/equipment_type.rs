use super::{equipment_connector, svg};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
//TODO: Make some of these fields enums
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct EquipmentType {
    pub manufacturer: String,
    pub model: String,
    pub part_number: String,
    pub manufacturer_part_number: String,
    pub supplier: String,
    pub supplier_part_number: String,
    pub mount_type: Vec<String>,
    pub equip_type: String,
    pub faces: HashMap<String, svg::Svg>,
    pub visual_rep: svg::Svg,
    pub connectors: HashMap<String, equipment_connector::EquipmentConnector>,
}
