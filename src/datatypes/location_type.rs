use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LocationType {
    pub manufacturer: String,
    pub model: String,
    pub part_number: String,
    pub manufacturer_part_number: String,
    pub supplier: String,
    pub supplier_part_number: String,
    pub material: String,
}
