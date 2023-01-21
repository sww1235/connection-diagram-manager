use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use super::equipment_type;

/// `Equipment` represents a particular instance of an `EquipmentType`.
/// This is the physical unit you would hold in your hand
///
/// Not all fields have to be populated, and some are
/// mainly provided for logical reasons rather than
/// functional (model/part number/manufacturer part number
/// may all be equivalent in some cases)
#[derive(Debug, Default)]

pub struct Equipment {
    //TODO figure out how to convert from
    //string specified in yaml file to
    //actual equipment type
    /// The type of equipment of the instance
    pub equip_type: Option<equipment_type::EquipmentType>,
    /// The string key of the instance_type. Converted into the actual type during the validation
    /// step of the parsing.
    equip_type_string: Option<String>,
    /// The structured name of the equipment
    pub identifier: Option<String>,
    /// The particular mounting type of this instance
    /// must be in list of mounting types defined in `equip_type.mounting_type`
    pub mounting_type: Option<String>,
    /// The individual location
    pub location: Option<String>, //TODO: Change to Option<location::Location>
    /// Description
    pub description: Option<String>,
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

impl fmt::Display for Equipment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Wire Type:")?;
        if let Some(equip_type) = &self.equip_type {
            if let Some(manufacturer) = &equip_type.manufacturer {
                writeln!(f, "Manufacturer: {}", manufacturer)?;
            }
            //TODO: Decide how much data from Equiptype we want to display for instance
            if let Some(model) = &equip_type.model {
                writeln!(f, "Model: {}", model)?;
            }
            if let Some(part_number) = &equip_type.part_number {
                writeln!(f, "Part Number: {}", part_number)?;
            }
            if let Some(manufacturer_part_number) = &equip_type.manufacturer_part_number {
                writeln!(f, "Manufacturer Part Number: {}", manufacturer_part_number)?;
            }
            if let Some(supplier) = &equip_type.supplier {
                writeln!(f, "Supplier: {}", supplier)?;
            }
            if let Some(supplier_part_number) = &equip_type.supplier_part_number {
                writeln!(f, "Supplier Part Number: {}", supplier_part_number)?;
            }
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
