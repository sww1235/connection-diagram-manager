use super::super::util_types::CrossSection;

use super::wire_type::WireType;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use std::rc::Rc;

use dimensioned::ucum;

use cdm_macros::{Empty, Merge, PartialEmpty};

/// `CableType` represents a type of cable that consists of multiple cores. If something only has one
/// core, then it is a wire, not a cable.
#[derive(Debug, Default, PartialEq, Merge, PartialEmpty, Empty)]
pub struct CableType {
    /// Unique ID of `CableType`
    pub id: String,
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
    pub cross_sect_area: ucum::Meter2<f64>,
    /// Cable cross section shape
    ///
    /// Oval, circular, siamese
    pub cross_section: CrossSection,
    /// height of cable in mm
    pub height: ucum::Meter<f64>,
    /// width of cable in mm
    pub width: ucum::Meter<f64>,
    /// diameter of cable in mm
    pub diameter: Option<ucum::Meter<f64>>,
    /// map of cores in cable
    pub cable_cores: HashMap<String, CableCore>,
    /// vector of exterior insulation/shielding layers
    pub insul_layers: Vec<CableLayer>,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}

//https://stackoverflow.com/questions/67594909/multiple-possible-types-for-a-serializable-structs-field

/// `CableCore` represents an individual conductor, strength member or optical fiber in a cable.
#[derive(Debug, PartialEq, Clone)]
#[allow(clippy::exhaustive_enums)]
pub enum CableCore {
    /// `WireType`
    WireType(Rc<RefCell<WireType>>),
    /// `CableType`
    CableType(Rc<RefCell<CableType>>),
}

/// `CableLayer` represents an insulation or shield layer of the entire cable
#[derive(Debug, Default, PartialEq, Clone)]
pub struct CableLayer {
    /// layer number, counted from inside to outside of cable, 1 indexed
    pub layer_number: u64,
    /// layer type
    pub layer_type: LayerType,
    /// `Material of CableLayer`
    pub material: Option<String>,
    /// Voltage rating for insuation layer
    pub volt_rating: Option<ucum::MilliVolt<f64>>,
    /// Temperature rating for insulation layer, specified in K
    pub temp_rating: Option<ucum::Kelvin<f64>>,
    /// color of `CableLayer`
    pub color: Option<String>,
}

/// `LayerType` represents different functions of a `CableLayer` `layer_type`
#[derive(Debug, Default, Clone, PartialEq)]
#[non_exhaustive]
pub enum LayerType {
    /// `Insulation` is a normal insulation layer of a cable
    #[default]
    Insulation,
    /// `Semiconductor` is a semiconducting layer in high voltage cables
    Semiconductor,
    /// `Shield` is a solid metallic shield for EMI reduction
    Shield,
    /// `Screen` is a mesh-like metallic shield for EMI reduction
    Screen,
    /// `ConcentricNeutral` is the outer concetric metallic neutral in high voltage cables
    ConcentricNeutral,
    /// `Armor` is a strength member that resists mechanical damage and is not designed as part of
    /// the electrical paths in the cable
    Armor,
}

impl CableType {
    /// Creates an empty instance of `CableType`
    #[allow(clippy::arithmetic_side_effects)]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl fmt::Display for CableType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Cable Type:")?;
        if let Some(manufacturer) = &self.manufacturer {
            writeln!(f, "Manufacturer: {manufacturer}")?;
        }
        if let Some(model) = &self.model {
            writeln!(f, "Model: {model}")?;
        }
        if let Some(part_number) = &self.part_number {
            writeln!(f, "Part Number: {part_number}")?;
        }
        if let Some(manufacturer_part_number) = &self.manufacturer_part_number {
            writeln!(f, "Manufacturer Part Number: {manufacturer_part_number}")?;
        }
        if let Some(supplier) = &self.supplier {
            writeln!(f, "Supplier: {supplier}")?;
        }
        if let Some(supplier_part_number) = &self.supplier_part_number {
            writeln!(f, "Supplier Part Number: {supplier_part_number}")?;
        }
        if let Some(cable_type_code) = &self.cable_type_code {
            writeln!(f, "Cable Type: {cable_type_code}")?;
        }
        if f.alternate() {
            //TODO: implement mm^2 to awg conversion function. include function for changing units
            writeln!(f, "Cross Sectional Area: {:.2} AWG", &self.cross_sect_area)?;
        } else {
            writeln!(f, "Cross Sectional Area: {:.2} mm^2", &self.cross_sect_area)?;
        }

        writeln!(f, "Cross Section: {}", &self.cross_section)?;
        writeln!(f, "Height: {:.2} mm", &self.height)?;
        writeln!(f, "Width: {:.2} mm", &self.width)?;
        if let Some(diameter) = &self.diameter {
            writeln!(f, "Diameter: {diameter:.2} mm")?;
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
