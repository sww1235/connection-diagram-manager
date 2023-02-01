use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use super::pathway_type::PathwayType;

/// `Pathway` represents a physical instance of a pathway
#[derive(Debug, Default)]
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
