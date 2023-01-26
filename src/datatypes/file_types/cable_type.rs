use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::fmt;

// TODO: allow for multiple cables inside cable
/// `CableType` represents a type of cable that consists of multiple cores. If something only has one
/// core, then it is a wire, not a cable.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CableType {
    /// Manufacturer of Cable
    pub manufacturer: Option<String>,
    /// Model of Cable
    pub model: Option<String>,
    /// Part number of Cable
    pub part_number: Option<String>,
    /// Manufacturer's Part Number
    pub manufacturer_part_number: Option<String>,
    /// Supplier of cable
    pub supplier: Option<String>,
    /// Supplier's part number
    pub supplier_part_number: Option<String>,
    /// Cable Type Code
    ///
    /// SOOW, NM, USE, etc
    pub cable_type_code: Option<String>,
    /// Cable cross sectional area, in mm^2
    pub cross_sect_area: f64,
    /// Cable cross section shape
    ///
    /// Oval, circular, siamese
    pub cross_section: String,
    /// height of cable in mm
    pub height: f64,
    /// width of cable in mm
    pub width: f64,
    /// diameter of cable in mm
    pub diameter: Option<f64>,
    /// map of cores in cable
    pub cable_core: HashMap<String, String>,
    /// vector of exterior insulation/shielding layers
    pub insul_layers: Vec<CableLayer>,
}

/// `CableLayer` represents an insulation or shield layer of the entire cable
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CableLayer {
    /// layer number, counted from inside to outside of cable, 1 indexed
    pub layer_number: Option<u64>,
    /// Insulation, Semiconductor, shield, screen, concentric neutral. TODO: change this to Enum
    pub layer_type: Option<String>,
    /// `Material of CableLayer`
    pub material: Option<String>,
    /// Voltage rating for insuation layer
    pub volt_rating: Option<u64>,
    /// Temperature rating for insulation layer, specified in C TODO: fix this with proper unicode
    pub temp_rating: Option<u64>,
    /// color of CableLayer
    pub color: Option<String>,
}

impl fmt::Display for CableType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Cable Type:")?;
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
        if let Some(cable_type_code) = &self.cable_type_code {
            writeln!(f, "\tCable Type: {}", cable_type_code)?;
        }
        if f.alternate() {
            //TODO: implement mm^2 to awg conversion function. include function for changing units
            writeln!(
                f,
                "\tCross Sectional Area: {:.2} AWG",
                &self.cross_sect_area
            )?;
        } else {
            writeln!(
                f,
                "\tCross Sectional Area: {:.2} mm^2",
                &self.cross_sect_area
            )?;
        }
        writeln!(f, "\tCross Section: {}", &self.cross_section)?;
        writeln!(f, "\tHeight: {:.2} mm", &self.height)?;
        writeln!(f, "\tWidth: {:.2} mm", &self.width)?;
        if let Some(diameter) = &self.diameter {
            writeln!(f, "\tDiameter: {:.2} mm", diameter)?;
        }
        //TODO: implement loops here to print all layers of cable
        //if let Some() = &self.model {
        //    writeln!(f, "Model: {}", )?;
        //}
        //if let Some() = &self.model {
        //    writeln!(f, "Model: {}", )?;
        //}
        Ok(())
    }
}
