use serde::{Deserialize, Serialize};

use std::fmt;

//TODO: move insulation color from wire to wire_type

/// `WireType` represents a particular type of wire
///
/// Not all fields have to be populated, and some are
/// mainly provided for logical reasons rather than
/// functional (model/part number/manufacturer part number
/// may all be equivalent in some cases)
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WireType {
    /// The manufacturer of the wire
    pub manufacturer: Option<String>,
    /// The model or type of wire
    pub model: Option<String>,
    /// The part number of this wire type
    pub part_number: Option<String>,
    /// The part number assigned by the manufacturer
    pub manufacturer_part_number: Option<String>,
    /// Where this wire was purchased from
    pub supplier: Option<String>,
    /// The part number the supplier uses
    pub supplier_part_number: Option<String>,
    /// The material the conductor or central element
    /// of the wire is made out of
    //TODO: rename this to account for fiber optics
    pub conductor_material: Option<String>,
    /// If the wire is insulated
    pub insulated: Option<bool>,
    /// What material the wire is insulated with
    pub insulation_material: Option<String>,
    /// The standard wire type code (THHN, XHHW, SIS, etc)
    pub wire_type_code: Option<String>,
    /// Conductor cross sectional area.
    /// specified in mm^2
    pub conductor_cross_sect_area: Option<f64>,
    /// Overall wire cross sectional area, incluidng insulation.
    /// specified in mm^2
    pub overall_cross_sect_area: Option<f64>,
    /// If conductor is stranded
    pub stranded: Option<bool>,
    /// How many strands is conductor made of
    pub num_strands: Option<u64>,
    /// cross sectional area of individual strand.
    /// specified in mm^2
    pub strand_cross_sect_area: Option<f64>,
    /// Insulation voltage rating.
    /// Specified in volts
    pub insul_volt_rating: Option<u64>,
    /// Insulation temperature rating.
    /// Specified in ℃
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
        if let Some(conductor_cross_sect_area) = &self.conductor_cross_sect_area {
            if f.alternate() {
                //TODO: implement mm^2 to AWG conversion, with auht and kcm display
                writeln!(
                    f,
                    "Conductor Cross Sectional Area: {} AWG",
                    conductor_cross_sect_area
                )?;
            } else {
                writeln!(
                    f,
                    "Conductor Cross Sectional Area: {:.2} mm^2",
                    conductor_cross_sect_area
                )?;
            }
        }
        if let Some(overall_cross_sect_area) = &self.overall_cross_sect_area {
            writeln!(
                f,
                "Overall Cross Sectional Area: {:.2} mm^2",
                overall_cross_sect_area
            )?;
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
