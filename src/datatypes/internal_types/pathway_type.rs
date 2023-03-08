use std::collections::HashMap;
use std::fmt;

use super::{Empty, Mergable, PartialEmpty};

/// PathwayType represents a route for wires and cables to take from one
/// [`LocationType`](super::location_type::LocationType) to another.
///
/// Examples of Pathways include, conduit, cable tray, free air
#[derive(Debug, Default, PartialEq)]
pub struct PathwayType {
    /// Internal ID of `PathwayType`
    pub id: String,
    /// Manufacturer of PathwayType
    pub manufacturer: Option<String>,
    /// Model of PathwayType
    pub model: Option<String>,
    /// Part Number of Pathway Type
    pub part_number: Option<String>,
    /// Manufacturer's Part Number of Pathway Type
    pub manufacturer_part_number: Option<String>,
    /// Supplier of Pathway Type
    pub supplier: Option<String>,
    /// Supplier's Part Number of Pathway Type
    pub supplier_part_number: Option<String>,
    /// Optional description text
    pub description: Option<String>,
    /// actual size of pathway
    pub size: Option<String>,
    /// Trade Size of pathway
    pub trade_size: Option<String>,
    //TODO: add in height, width, etc
    /// Inner cross sectional area of pathway
    pub cross_sect_area: f64,
    /// Main material of pathway
    pub material: Option<String>,
}
impl PathwayType {
    /// Creates an empty instance of `PathwayType`
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
            size: None,
            trade_size: None,
            cross_sect_area: 0.0,
            material: None,
        }
    }
}

impl Mergable for PathwayType {
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
        if self.size != other.size {
            input_map.insert(
                "Size".to_string(),
                [
                    {
                        if let Some(size) = self.size.clone() {
                            size
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(size) = other.size.clone() {
                            size
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.trade_size != other.trade_size {
            input_map.insert(
                "Trade Size".to_string(),
                [
                    {
                        if let Some(trade_size) = self.trade_size.clone() {
                            trade_size
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(trade_size) = other.trade_size.clone() {
                            trade_size
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
        if results["Size"] {
            self.size = other.size.clone();
        }
        if results["Trade Size"] {
            self.trade_size = other.trade_size.clone();
        }
        if results["Cross Sectional Area"] {
            self.cross_sect_area = other.cross_sect_area;
        }
        if results["Material"] {
            self.material = other.material.clone();
        }
    }
}

impl Empty for PathwayType {
    fn is_empty(&self) -> bool {
        self == &Self::new()
    }
}

impl PartialEmpty for PathwayType {
    fn is_partial_empty(&self) -> bool {
        let tester = Self::new();
        self.manufacturer == tester.manufacturer
            && self.model == tester.model
            && self.part_number == tester.part_number
            && self.manufacturer_part_number == tester.manufacturer_part_number
            && self.supplier == tester.supplier
            && self.supplier_part_number == tester.supplier_part_number
            && self.description == tester.description
            && self.size == tester.size
            && self.trade_size == tester.trade_size
            && self.cross_sect_area == tester.cross_sect_area
            && self.material == tester.material
    }
}

impl fmt::Display for PathwayType {
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
        if let Some(size) = &self.size {
            writeln!(f, "Size: {size}")?;
        }
        if let Some(trade_size) = &self.trade_size {
            writeln!(f, "Trade Size: {trade_size}")?;
        }
        //TODO: implement unit conversion function
        writeln!(f, "Cross Sectional Area: {:.2} mm^2", self.cross_sect_area)?;
        if let Some(material) = &self.material {
            writeln!(f, "Material: {material}")?;
        }
        Ok(())
    }
}
