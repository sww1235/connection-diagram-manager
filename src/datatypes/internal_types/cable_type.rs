use super::super::util_types::CrossSection;

use super::{wire_type::WireType, Empty, Mergable, PartialEmpty};

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

/// `CableType` represents a type of cable that consists of multiple cores. If something only has one
/// core, then it is a wire, not a cable.
#[derive(Debug, Default, PartialEq)]
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
    pub cross_sect_area: f64,
    /// Cable cross section shape
    ///
    /// Oval, circular, siamese
    pub cross_section: CrossSection,
    /// height of cable in mm
    pub height: f64,
    /// width of cable in mm
    pub width: f64,
    /// diameter of cable in mm
    pub diameter: Option<f64>,
    /// map of cores in cable
    pub cable_core: HashMap<String, CableCore>,
    /// vector of exterior insulation/shielding layers
    pub insul_layers: Vec<CableLayer>,
}

//https://stackoverflow.com/questions/67594909/multiple-possible-types-for-a-serializable-structs-field

/// `CableCore` represents an individual conductor, strength member or optical fiber in a cable.
#[derive(Debug, PartialEq)]
pub enum CableCore {
    /// `WireType`
    WireType(Rc<RefCell<WireType>>),
    /// `CableType`
    CableType(Rc<RefCell<CableType>>),
}

/// `CableLayer` represents an insulation or shield layer of the entire cable
#[derive(Debug, Default, PartialEq)]
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

impl CableType {
    /// Creates an empty instance of `CableType`
    pub fn new() -> Self {
        Self {
            id: String::new(),
            manufacturer: None,
            model: None,
            part_number: None,
            manufacturer_part_number: None,
            supplier: None,
            supplier_part_number: None,
            cable_type_code: None,
            cross_sect_area: 0.0,
            cross_section: CrossSection::default(),
            height: 0.0,
            width: 0.0,
            diameter: None,
            cable_core: HashMap::new(),
            insul_layers: Vec::new(),
        }
    }
}

impl Mergable for CableType {
    fn merge_prompt(
        &mut self,
        other: &Self,
        prompt_fn: fn(HashMap<String, [String; 2]>) -> HashMap<String, u8>,
    ) -> Self {
        todo!();
    }
}

impl Empty for CableType {
    fn is_empty(&self) -> bool {
        self == &Self::new()
    }
}

impl PartialEmpty for CableType {
    fn is_partial_empty(&self) -> bool {
        let tester = Self::new();
        self.manufacturer == tester.manufacturer
            && self.model == tester.model
            && self.part_number == tester.part_number
            && self.manufacturer_part_number == tester.manufacturer_part_number
            && self.supplier == tester.supplier
            && self.supplier_part_number == tester.supplier_part_number
            && self.cable_type_code == tester.cable_type_code
            && self.cross_sect_area == tester.cross_sect_area
            && self.cross_section == tester.cross_section
            && self.height == tester.height
            && self.width == tester.width
            && self.diameter == tester.diameter
            && self.cable_core == tester.cable_core
            && self.insul_layers == tester.insul_layers
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
