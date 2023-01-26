use serde::{Deserialize, Serialize};

use std::fmt;

//TODO: move insulation color from wire to wire_type

/// `WireType` represents a particular type of wire
///
/// Not all fields have to be populated, and some are
/// mainly provided for logical reasons rather than
/// functional (model/part number/manufacturer part number
/// may all be equivalent in some cases)
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
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
    pub insulated: bool,
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
    pub stranded: bool,
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
    /// Insulation color
    pub insul_color: Option<String>,
}
impl fmt::Display for WireType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Wire Type:")?;
        if let Some(manufacturer) = &self.manufacturer {
            writeln!(f, "\tManufacturer: {}", manufacturer)?;
        }
        if let Some(model) = &self.model {
            writeln!(f, "\tModel: {}", model)?;
        }
        if let Some(part_number) = &self.part_number {
            writeln!(f, "\tPart Number: {}", part_number)?;
        }
        if let Some(manufacturer_part_number) = &self.manufacturer_part_number {
            writeln!(
                f,
                "\tManufacturer Part Number: {}",
                manufacturer_part_number
            )?;
        }
        if let Some(supplier) = &self.supplier {
            writeln!(f, "\tSupplier: {}", supplier)?;
        }
        if let Some(supplier_part_number) = &self.supplier_part_number {
            writeln!(f, "\tSupplier Part Number: {}", supplier_part_number)?;
        }
        if let Some(conductor_material) = &self.conductor_material {
            writeln!(f, "\tConductor Material: {}", conductor_material)?;
        }
        writeln!(f, "\tInsulated: {}", &self.insulated)?;
        if let Some(insulation_material) = &self.insulation_material {
            writeln!(f, "\tInsulation Material: {}", insulation_material)?;
        }
        if let Some(wire_type_code) = &self.wire_type_code {
            writeln!(f, "\tWire Type Code: {}", wire_type_code)?;
        }
        if let Some(conductor_cross_sect_area) = &self.conductor_cross_sect_area {
            if f.alternate() {
                //TODO: implement mm^2 to AWG conversion, with auht and kcm display
                writeln!(
                    f,
                    "\tConductor Cross Sectional Area: {} AWG",
                    conductor_cross_sect_area
                )?;
            } else {
                writeln!(
                    f,
                    "\tConductor Cross Sectional Area: {:.2} mm^2",
                    conductor_cross_sect_area
                )?;
            }
        }
        if let Some(overall_cross_sect_area) = &self.overall_cross_sect_area {
            writeln!(
                f,
                "\tOverall Cross Sectional Area: {:.2} mm^2",
                overall_cross_sect_area
            )?;
        }
        writeln!(f, "\tStranded: {}", &self.stranded)?;
        if let Some(num_strands) = &self.num_strands {
            writeln!(f, "\tNumber of Strands: {}", num_strands)?;
        }
        if let Some(strand_cross_sect_area) = &self.strand_cross_sect_area {
            if f.alternate() {
                //TODO: implement mm^2 to AWG conversion, with auht and kcm display
                writeln!(
                    f,
                    "\tStrand Cross Sectional Area: {} AWG",
                    strand_cross_sect_area
                )?;
            } else {
                writeln!(
                    f,
                    "\tStrand Cross Sectional Area: {:.2} mm^2",
                    strand_cross_sect_area
                )?;
            }
        }
        if let Some(insul_volt_rating) = &self.insul_volt_rating {
            writeln!(f, "\tInsulation Voltage Rating: {}V", insul_volt_rating)?;
        }
        if let Some(insul_temp_rating) = &self.insul_temp_rating {
            writeln!(f, "\tInsulation Temperature Rating: {}℃", insul_temp_rating)?;
        }
        if let Some(insul_color) = &self.insul_color {
            writeln!(f, "\tInsulation Color: {}", insul_color)?;
        }
        Ok(())
    }
}
