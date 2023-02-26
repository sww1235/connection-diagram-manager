use std::collections::HashMap;
use std::fmt;

use super::{Empty, Mergable, PartialEmpty};

//TODO: create physical location stuff
/// LocationType represents a type/model of location.
///
/// Examples of LocationType include junction boxes, racks, panels, etc.
/// It does not include places these are located.
#[derive(Debug, Default, PartialEq)]
pub struct LocationType {
    /// Internal ID of `LocationType`
    pub id: String,
    /// Manufacturer of LocationType
    pub manufacturer: Option<String>,
    /// Model of LocationType
    pub model: Option<String>,
    /// Part Number of LocationType
    pub part_number: Option<String>,
    /// Manufacturer's Part Number
    pub manufacturer_part_number: Option<String>,
    /// Supplier of Part Number
    pub supplier: Option<String>,
    /// Supplier's Part Number
    pub supplier_part_number: Option<String>,
    /// Optional description
    pub description: Option<String>,
    /// Main material of LocationType
    pub material: Option<String>,
    /// Width of locationType
    pub width: f64,
    /// Height of locationType
    pub height: f64,
    /// Depth of locationType
    pub depth: f64,
    /// Usable Width of locationType
    pub usable_width: f64,
    /// Usable Height of locationType
    pub usable_height: f64,
    /// Usable Depth of locationType
    pub usable_depth: f64,
}

impl LocationType {
    /// Creates an empty instance of `LocationType`
    pub fn new() -> Self {
        Self {
            id: String::new(),
            manufacturer: None,
            model: None,
            part_number: None,
            manufacturer_part_number: None,
            supplier: None,
            supplier_part_number: None,
            description: None,
            material: None,
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            usable_width: 0.0,
            usable_height: 0.0,
            usable_depth: 0.0,
        }
    }
}
impl Mergable for LocationType {
    fn merge_prompt(
        &mut self,
        other: &Self,
        prompt_fn: fn(HashMap<String, [String; 2]>) -> HashMap<String, u8>,
    ) -> Self {
        todo!();
    }
}

impl Empty for LocationType {
    fn is_empty(&self) -> bool {
        self == &Self::new()
    }
}

impl PartialEmpty for LocationType {
    fn is_partial_empty(&self) -> bool {
        let tester = Self::new();
        self.manufacturer == tester.manufacturer
            && self.model == tester.model
            && self.part_number == tester.part_number
            && self.manufacturer_part_number == tester.manufacturer_part_number
            && self.supplier == tester.supplier
            && self.supplier_part_number == tester.supplier_part_number
            && self.description == tester.description
            && self.height == tester.height
            && self.width == tester.width
            && self.depth == tester.depth
            && self.usable_height == tester.usable_height
            && self.usable_width == tester.usable_width
            && self.usable_depth == tester.usable_depth
    }
}

impl fmt::Display for LocationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Connector Type:")?;
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
        if let Some(description) = &self.description {
            writeln!(f, "Description: {description}")?;
        }
        if let Some(material) = &self.material {
            writeln!(f, "Material: {material}")?;
        }
        writeln!(f, "Width: {}", self.width)?;
        writeln!(f, "Height: {}", self.height)?;
        writeln!(f, "Depth: {}", self.depth)?;
        writeln!(f, "Usable Width: {}", self.usable_width)?;
        writeln!(f, "Usable Height: {}", self.usable_height)?;
        writeln!(f, "Usable Depth: {}", self.usable_depth)?;
        Ok(())
    }
}
