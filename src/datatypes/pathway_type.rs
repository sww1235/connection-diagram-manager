use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PathwayType {
    pub manufacturer: String,
    pub model: String,
    pub part_number: String,
    pub manufacturer_part_number: String,
    pub supplier: String,
    pub supplier_part_number: String,
    pub size: String,
    pub trade_size: String,
    //TODO: add in height, width, etc
    pub cross_sect_area: f64,
    pub material: String,
}
