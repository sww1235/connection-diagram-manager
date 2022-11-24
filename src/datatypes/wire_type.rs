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
        writeln!(f, "Wire Type:")?;
        if let Some(manufacturer) = &self.manufacturer {
            writeln!(f, "Manufacturer: {}", manufacturer)?;
        }
        if let Some(model) = &self.model {
            writeln!(f, "Model: {}", model)?;
        }
        if let Some(part_number) = &self.part_number {
            writeln!(f, "Part Number: {}", part_number)?;
        }
        if let Some(manufacturer_part_number) = &self.manufacturer_part_number {
            writeln!(f, "Manufacturer Part Number: {}", manufacturer_part_number)?;
        }
        if let Some(supplier) = &self.supplier {
            writeln!(f, "Supplier: {}", supplier)?;
        }
        if let Some(supplier_part_number) = &self.supplier_part_number {
            writeln!(f, "Supplier Part Number: {}", supplier_part_number)?;
        }
        if let Some(conductor_material) = &self.conductor_material {
            writeln!(f, "Conductor Material: {}", conductor_material)?;
        }
        if let Some(insulated) = &self.insulated {
            writeln!(f, "Insulated: {}", insulated)?;
        }
        if let Some(insulation_material) = &self.insulation_material {
            writeln!(f, "Insulation Material: {}", insulation_material)?;
        }
        if let Some(wire_type_code) = &self.wire_type_code {
            writeln!(f, "Wire Type Code: {}", wire_type_code)?;
        }
        if let Some(cross_sect_area) = &self.cross_sect_area {
            if f.alternate() {
                //TODO: implement mm^2 to AWG conversion, with auht and kcm display
                writeln!(f, "Cross Sectional Area: {} AWG", cross_sect_area)?;
            } else {
                writeln!(f, "Cross Sectional Area: {:.2} mm^2", cross_sect_area)?;
            }
        }
        if let Some(stranded) = &self.stranded {
            writeln!(f, "Stranded: {}", stranded)?;
        }
        if let Some(num_strands) = &self.num_strands {
            writeln!(f, "Number of Strands: {}", num_strands)?;
        }
        if let Some(strand_cross_sect_area) = &self.strand_cross_sect_area {
            if f.alternate() {
                //TODO: implement mm^2 to AWG conversion, with auht and kcm display
                writeln!(
                    f,
                    "Strand Cross Sectional Area: {} AWG",
                    strand_cross_sect_area
                )?;
            } else {
                writeln!(
                    f,
                    "Strand Cross Sectional Area: {:.2} mm^2",
                    strand_cross_sect_area
                )?;
            }
        }
        if let Some(insul_volt_rating) = &self.insul_volt_rating {
            writeln!(f, "Insulation Voltage Rating: {}V", insul_volt_rating)?;
        }
        if let Some(insul_temp_rating) = &self.insul_temp_rating {
            writeln!(f, "Insulation Temperature Rating: {}℃", insul_temp_rating)?;
        }
        Ok(())
    }
}
