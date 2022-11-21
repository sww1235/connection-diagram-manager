use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PathwayType {
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub part_number: Option<String>,
    pub manufacturer_part_number: Option<String>,
    pub supplier: Option<String>,
    pub supplier_part_number: Option<String>,
    pub size: Option<String>,
    pub trade_size: Option<String>,
    //TODO: add in height, width, etc
    pub cross_sect_area: Option<f64>,
    pub material: Option<String>,
}
