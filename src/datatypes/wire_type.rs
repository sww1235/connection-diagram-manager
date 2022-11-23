use serde::{Deserialize, Serialize};

use std::fmt;

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
    pub insul_volt_rating: Option<u64>,
    pub insul_temp_rating: Option<u64>,
}
impl fmt::Display for WireType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Wire Type:")?;
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
        if let Some(foo) = &self.conductor_material {
            write!(f, "Conductor Material: {}", foo)?;
        }
        if let Some(foo) = &self.insulated {
            write!(f, "Insulated: {}", foo)?;
        }
        if let Some(foo) = &self.insulation_material {
            write!(f, "Insulation Material: {}", foo)?;
        }
        if let Some(foo) = &self.wire_type_code {
            write!(f, "Wire Type Code: {}", foo)?;
        }
        if let Some(foo) = &self.cross_sect_area {
            if f.alternate() {
                //TODO: implement mm^2 to AWG conversion, with auht and kcm display
                write!(f, "Cross Sectional Area: {} AWG", foo)?;
            } else {
                write!(f, "Cross Sectional Area: {:.2} mm^2", foo)?;
            }
        }
        if let Some(foo) = &self.stranded {
            write!(f, "Stranded: {}", foo)?;
        }
        if let Some(foo) = &self.num_strands {
            write!(f, "Number of Strands: {}", foo)?;
        }
        if let Some(foo) = &self.strand_cross_sect_area {
            if f.alternate() {
                //TODO: implement mm^2 to AWG conversion, with auht and kcm display
                write!(f, "Strand Cross Sectional Area: {} AWG", foo)?;
            } else {
                write!(f, "Strand Cross Sectional Area: {:.2} mm^2", foo)?;
            }
        }
        if let Some(foo) = &self.insul_volt_rating {
            write!(f, "Insulation Voltage Rating: {}V", foo)?;
        }
        if let Some(foo) = &self.insul_temp_rating {
            write!(f, "Insulation Temperature Rating: {}℃", foo)?;
        }
        Ok(())
    }
}
