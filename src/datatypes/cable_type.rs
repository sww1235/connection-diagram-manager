use serde::{Deserialize, Serialize};

use std::collections::HashMap;
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CableType {
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub part_number: Option<String>,
    pub manufacturer_part_number: Option<String>,
    pub supplier: Option<String>,
    pub supplier_part_number: Option<String>,
    pub cable_type_code: Option<String>,
    pub cross_sect_area: Option<f64>,
    pub cross_section: Option<String>,
    pub height: Option<f64>,
    pub width: Option<f64>,
    pub diameter: Option<f64>,
    pub cable_core: Option<HashMap<String, CableCore>>,
    pub layers: Option<Vec<CableLayer>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CableCore {
    pub core_type: Option<String>,
    pub color: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CableLayer {
    pub layer_number: Option<u64>,
    pub layer_type: Option<String>,
    pub material: Option<String>,
    pub volt_rating: Option<f64>,
    pub temp_rating: Option<f64>,
    pub color: Option<String>,
}
