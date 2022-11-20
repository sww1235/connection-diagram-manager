use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WireType {
    pub manufacturer: String,
    pub model: String,
    pub part_number: String,
    pub manufacturer_part_number: String,
    pub supplier: String,
    pub supplier_part_number: String,
    pub conductor_material: String,
    pub insulated: bool,
    pub insulation_material: String,
    pub wire_type_code: String,
    pub cross_sect_area: f64,
    pub stranded: bool,
    pub num_strands: u64,
    pub strand_cross_sect_area: f64,
    pub insul_volt_rating: f64,
    pub insul_temp_rating: f64,
}
