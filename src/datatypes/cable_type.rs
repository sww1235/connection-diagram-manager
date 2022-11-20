use serde::{Deserialize, Serialize};

use std::collections::HashMap;
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CableType {
    pub manufacturer: String,
    pub model: String,
    pub part_number: String,
    pub manufacturer_part_number: String,
    pub supplier: String,
    pub supplier_part_number: String,
    pub cable_type_code: String,
    pub cross_sect_area: f64,
    pub cross_section: String,
    pub height: f64,
    pub width: f64,
    pub diameter: f64,
    pub cable_core: HashMap<String, CableCore>,
    pub layers: Vec<CableLayer>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CableCore {
    pub core_type: String,
    pub color: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CableLayer {
    pub layer_number: u64,
    pub layer_type: String,
    pub material: String,
    pub volt_rating: f64,
    pub temp_rating: f64,
    pub color: String,
}
