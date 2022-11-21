use super::{equipment_connector, svg};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
//TODO: Make some of these fields enums
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct EquipmentType {
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub part_number: Option<String>,
    pub manufacturer_part_number: Option<String>,
    pub supplier: Option<String>,
    pub supplier_part_number: Option<String>,
    pub mount_type: Option<Vec<String>>,
    pub equip_type: Option<String>,
    pub faces: Option<HashMap<String, svg::Svg>>,
    pub visual_rep: Option<svg::Svg>,
    pub connectors: Option<HashMap<String, equipment_connector::EquipmentConnector>>,
}
