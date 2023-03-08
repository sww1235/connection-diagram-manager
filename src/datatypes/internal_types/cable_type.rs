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
    pub cable_cores: HashMap<String, CableCore>,
    /// vector of exterior insulation/shielding layers
    pub insul_layers: Vec<CableLayer>,
}

//https://stackoverflow.com/questions/67594909/multiple-possible-types-for-a-serializable-structs-field

/// `CableCore` represents an individual conductor, strength member or optical fiber in a cable.
#[derive(Debug, PartialEq, Clone)]
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
    /// Insulation, Semiconductor, shield, screen, concentric neutral. TODO: change this to Enum
    pub layer_type: String,
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
            cable_cores: HashMap::new(),
            insul_layers: Vec::new(),
        }
    }
}

impl Mergable for CableType {
    fn merge_prompt(
        &mut self,
        other: &Self,
        prompt_fn: fn(HashMap<String, [String; 2]>) -> HashMap<String, bool>,
    ) {
        //TODO: maybe check for partial_empty/empty here on other
        let mut input_map: HashMap<String, [String; 2]> = HashMap::new();
        if self.id != other.id {
            panic! {"attempting to merge structs with different IDs. This shouldn't have happened."}
        }
        if self.manufacturer != other.manufacturer {
            input_map.insert(
                "Manufacturer".to_string(),
                [
                    {
                        if let Some(manufacturer) = self.manufacturer.clone() {
                            manufacturer
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(manufacturer) = other.manufacturer.clone() {
                            manufacturer
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.model != other.model {
            input_map.insert(
                "Model".to_string(),
                [
                    {
                        if let Some(model) = self.model.clone() {
                            model
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(model) = other.model.clone() {
                            model
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.part_number != other.part_number {
            input_map.insert(
                "Part Number".to_string(),
                [
                    {
                        if let Some(part_number) = self.part_number.clone() {
                            part_number
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(part_number) = other.part_number.clone() {
                            part_number
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.manufacturer_part_number != other.manufacturer_part_number {
            input_map.insert(
                "Manufacturer Part Number".to_string(),
                [
                    {
                        if let Some(manufacturer_part_number) =
                            self.manufacturer_part_number.clone()
                        {
                            manufacturer_part_number
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(manufacturer_part_number) =
                            other.manufacturer_part_number.clone()
                        {
                            manufacturer_part_number
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.supplier != other.supplier {
            input_map.insert(
                "Supplier".to_string(),
                [
                    {
                        if let Some(supplier) = self.supplier.clone() {
                            supplier
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(supplier) = other.supplier.clone() {
                            supplier
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.supplier_part_number != other.supplier_part_number {
            input_map.insert(
                "Supplier Part Number".to_string(),
                [
                    {
                        if let Some(supplier_part_number) = self.supplier_part_number.clone() {
                            supplier_part_number
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(supplier_part_number) = other.supplier_part_number.clone() {
                            supplier_part_number
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.cable_type_code != other.cable_type_code {
            input_map.insert(
                "Cable Type Code".to_string(),
                [
                    {
                        if let Some(cable_type_code) = self.cable_type_code.clone() {
                            cable_type_code
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(cable_type_code) = other.cable_type_code.clone() {
                            cable_type_code
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.cross_sect_area != other.cross_sect_area {
            input_map.insert(
                "Cross Sectional Area".to_string(),
                [
                    self.cross_sect_area.to_string(),
                    other.cross_sect_area.to_string(),
                ],
            );
        }
        if self.cross_section != other.cross_section {
            input_map.insert(
                "Cross Section".to_string(),
                [
                    self.cross_section.to_string(),
                    other.cross_section.to_string(),
                ],
            );
        }
        if self.height != other.height {
            input_map.insert(
                "Height".to_string(),
                [self.height.to_string(), other.height.to_string()],
            );
        }
        if self.width != other.width {
            input_map.insert(
                "Width".to_string(),
                [self.width.to_string(), other.width.to_string()],
            );
        }
        if self.diameter != other.diameter {
            input_map.insert(
                "Diameter".to_string(),
                [
                    {
                        if let Some(diameter) = self.diameter {
                            diameter.to_string()
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(diameter) = other.diameter {
                            diameter.to_string()
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.cable_cores != other.cable_cores {
            let mut self_string = String::new();
            let mut other_string = String::new();
            for core in self.cable_cores.keys() {
                self_string = self_string + core + "\t"
            }
            for core in other.cable_cores.keys() {
                other_string = other_string + core
            }
            input_map.insert("Cable Cores".to_string(), [self_string, other_string]);
        }
        if self.insul_layers != other.insul_layers {
            let mut self_string = String::new();
            let mut other_string = String::new();
            for layer in &self.insul_layers {
                self_string.push('(');
                if let Some(material) = &layer.material {
                    self_string.push_str(material.as_str());
                }
                self_string.push_str(", ");
                if let Some(color) = &layer.color {
                    self_string.push_str(color.as_str());
                }
                self_string.push_str(")\t")
            }
            for layer in &other.insul_layers {
                other_string.push('(');
                if let Some(material) = &layer.material {
                    other_string.push_str(material.as_str());
                }
                other_string.push_str(", ");
                if let Some(color) = &layer.color {
                    other_string.push_str(color.as_str());
                }
                other_string.push_str(")\t");
            }
            input_map.insert("Insulation Layers".to_string(), [self_string, other_string]);
        }

        let results = prompt_fn(input_map);
        // false means don't replace value in self struct
        if results["Manufacturer"] {
            self.manufacturer = other.manufacturer.clone();
        }
        if results["Model"] {
            self.model = other.model.clone();
        }
        if results["Part Number"] {
            self.part_number = other.part_number.clone();
        }
        if results["Manufacturer Part Number"] {
            self.manufacturer_part_number = other.manufacturer_part_number.clone();
        }
        if results["Supplier"] {
            self.supplier = other.supplier.clone();
        }
        if results["Supplier Part Number"] {
            self.supplier_part_number = other.supplier_part_number.clone();
        }
        if results["Cable Type Code"] {
            self.cable_type_code = other.cable_type_code.clone();
        }
        if results["Cross Sectional Area"] {
            self.cross_sect_area = other.cross_sect_area;
        }
        if results["Cross Section"] {
            self.cross_section = other.cross_section.clone();
        }
        if results["Height"] {
            self.height = other.height;
        }
        if results["Width"] {
            self.width = other.width;
        }
        if results["Diameter"] {
            self.diameter = other.diameter;
        }
        if results["Cable Cores"] {
            self.cable_cores = other.cable_cores.clone();
        }
        if results["Insulation Layers"] {
            self.insul_layers = other.insul_layers.clone();
        }
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
            && self.cable_cores == tester.cable_cores
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
