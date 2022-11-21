use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WireType {
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub part_number: Option<String>,
    pub manufacturer_part_number: Option<String>,
    pub supplier: Option<String>,
    pub supplier_part_number: Option<String>,
    pub conductor_material: Option<String>,
    pub insulated: Option<bool>,
    pub insulation_material: Option<String>,
    pub wire_type_code: Option<String>,
    pub cross_sect_area: Option<f64>,
    pub stranded: Option<bool>,
    pub num_strands: Option<u64>,
    pub strand_cross_sect_area: Option<f64>,
    pub insul_volt_rating: Option<f64>,
    pub insul_temp_rating: Option<f64>,
}
