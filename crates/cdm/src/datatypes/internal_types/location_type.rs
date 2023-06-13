use std::collections::HashMap;
use std::fmt;

use cdm_traits::{Empty, Mergable, PartialEmpty};

use dimensioned::ucum;

//TODO: create physical location stuff
/// `LocationType` represents a type/model of location.
///
/// Examples of `LocationType` include junction boxes, racks, panels, etc.
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
    pub width: ucum::Meter<f64>,
    /// Height of locationType
    pub height: ucum::Meter<f64>,
    /// Depth of locationType
    pub depth: ucum::Meter<f64>,
    /// Usable Width of locationType
    pub usable_width: ucum::Meter<f64>,
    /// Usable Height of locationType
    pub usable_height: ucum::Meter<f64>,
    /// Usable Depth of locationType
    pub usable_depth: ucum::Meter<f64>,
}

impl LocationType {
    /// Creates an empty instance of `LocationType`
    #[allow(clippy::arithmetic_side_effects)]
    #[must_use]
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
            width: 0.0_f64 * ucum::M,
            height: 0.0_f64 * ucum::M,
            depth: 0.0_f64 * ucum::M,
            usable_width: 0.0_f64 * ucum::M,
            usable_height: 0.0_f64 * ucum::M,
            usable_depth: 0.0_f64 * ucum::M,
        }
    }
}
impl Mergable for LocationType {
    #[allow(clippy::too_many_lines)]
    // TODO: see if this can be split up
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
        if self.description != other.description {
            input_map.insert(
                "Description".to_string(),
                [
                    {
                        if let Some(description) = self.description.clone() {
                            description
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(description) = other.description.clone() {
                            description
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.material != other.material {
            input_map.insert(
                "Material".to_string(),
                [
                    {
                        if let Some(material) = self.material.clone() {
                            material
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(material) = other.material.clone() {
                            material
                        } else {
                            String::new()
                        }
                    },
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
        if self.depth != other.depth {
            input_map.insert(
                "Depth".to_string(),
                [self.depth.to_string(), other.depth.to_string()],
            );
        }
        if self.usable_height != other.usable_height {
            input_map.insert(
                "Usable Height".to_string(),
                [
                    self.usable_height.to_string(),
                    other.usable_height.to_string(),
                ],
            );
        }
        if self.usable_width != other.usable_width {
            input_map.insert(
                "Usable Width".to_string(),
                [
                    self.usable_width.to_string(),
                    other.usable_width.to_string(),
                ],
            );
        }
        if self.usable_depth != other.usable_depth {
            input_map.insert(
                "Usable Depth".to_string(),
                [
                    self.usable_depth.to_string(),
                    other.usable_depth.to_string(),
                ],
            );
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
        if results["Description"] {
            self.description = other.description.clone();
        }
        if results["Material"] {
            self.material = other.material.clone();
        }
        if results["Height"] {
            self.height = other.height;
        }
        if results["Width"] {
            self.width = other.width;
        }
        if results["Depth"] {
            self.depth = other.depth;
        }
        if results["Usable Height"] {
            self.usable_height = other.usable_height;
        }
        if results["Usable Width"] {
            self.usable_width = other.usable_width;
        }
        if results["Usable Depth"] {
            self.usable_depth = other.usable_depth;
        }
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
