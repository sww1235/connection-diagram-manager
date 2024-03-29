use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use super::{pathway_type::PathwayType, Empty, Mergable, PartialEmpty};

/// `Pathway` represents a physical instance of a pathway
#[derive(Debug, Default, PartialEq)]
pub struct Pathway {
    /// Internal `id` of pathway instance
    pub id: String,
    /// Type of pathway
    pub path_type: Rc<RefCell<PathwayType>>,
    /// structured identifier of pathway
    pub identifier: Option<String>,
    /// Optional description
    pub description: Option<String>,
    /// length TODO: change to correct units
    pub length: f64,
}
impl Pathway {
    /// Creates an empty instance of `Pathway`
    pub fn new() -> Self {
        Self {
            id: String::new(),
            path_type: Rc::new(RefCell::new(PathwayType::new())),
            identifier: None,
            description: None,
            length: 0.0,
        }
    }
}

impl Mergable for Pathway {
    fn merge_prompt(
        &mut self,
        other: &Self,
        prompt_fn: fn(HashMap<String, [String; 2]>) -> HashMap<String, u8>,
    ) -> Self {
        todo!();
    }
}

impl Empty for Pathway {
    fn is_empty(&self) -> bool {
        self == &Self::new()
    }
}

impl PartialEmpty for Pathway {
    fn is_partial_empty(&self) -> bool {
        let tester = Self::new();
        self.path_type == tester.path_type
            && self.identifier == tester.identifier
            && self.length == tester.length
            && self.description == tester.description
    }
}

impl fmt::Display for Pathway {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Path Instance:")?;
        if let Some(manufacturer) = &self.path_type.borrow().manufacturer {
            writeln!(f, "Manufacturer: {manufacturer}")?;
        }
        //TODO: Decide how much data from Equiptype we want to display for instance
        if let Some(model) = &self.path_type.borrow().model {
            writeln!(f, "Model: {model}")?;
        }
        if let Some(part_number) = &self.path_type.borrow().part_number {
            writeln!(f, "Part Number: {part_number}")?;
        }
        if let Some(manufacturer_part_number) = &self.path_type.borrow().manufacturer_part_number {
            writeln!(f, "Manufacturer Part Number: {manufacturer_part_number}")?;
        }
        if let Some(supplier) = &self.path_type.borrow().supplier {
            writeln!(f, "Supplier: {supplier}")?;
        }
        if let Some(supplier_part_number) = &self.path_type.borrow().supplier_part_number {
            writeln!(f, "Supplier Part Number: {supplier_part_number}")?;
        }
        if let Some(identifier) = &self.identifier {
            writeln!(f, "Equipment Identifier: {identifier}")?;
        }
        writeln!(f, "Length: {}", &self.length)?;

        if let Some(description) = &self.description {
            writeln!(f, "Description: {description}")?;
        }
        Ok(())
    }
}
