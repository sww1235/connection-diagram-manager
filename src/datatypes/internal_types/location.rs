use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use super::location_type::LocationType;

/// `Location` represents a physical instance of a pathway
#[derive(Debug, Default, PartialEq)]
pub struct Location {
    /// Internal `id` of location instance
    pub id: String,
    /// Type of location
    pub location_type: Rc<RefCell<LocationType>>,
    /// structured identifier of location
    pub identifier: Option<String>,
    /// Optional description
    pub description: Option<String>,
    /// Physical Location
    pub physical_location: Option<String>,
    //TODO: add sub locations
}
impl Location {
    /// Creates an empty instance of `Location`
    pub fn new() -> Self {
        Self {
            id: String::new(),
            location_type: Rc::new(RefCell::new(LocationType::new())),
            identifier: None,
            description: None,
            physical_location: None,
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Location Instance:")?;
        writeln!(f, "Internal ID: {}", &self.id)?;
        if let Some(manufacturer) = &self.location_type.borrow().manufacturer {
            writeln!(f, "Manufacturer: {manufacturer}")?;
        }
        //TODO: Decide how much data from Equiptype we want to display for instance
        if let Some(model) = &self.location_type.borrow().model {
            writeln!(f, "Model: {model}")?;
        }
        if let Some(part_number) = &self.location_type.borrow().part_number {
            writeln!(f, "Part Number: {part_number}")?;
        }
        if let Some(manufacturer_part_number) =
            &self.location_type.borrow().manufacturer_part_number
        {
            writeln!(f, "Manufacturer Part Number: {manufacturer_part_number}")?;
        }
        if let Some(supplier) = &self.location_type.borrow().supplier {
            writeln!(f, "Supplier: {supplier}")?;
        }
        if let Some(supplier_part_number) = &self.location_type.borrow().supplier_part_number {
            writeln!(f, "Supplier Part Number: {supplier_part_number}")?;
        }
        if let Some(identifier) = &self.identifier {
            writeln!(f, "Location Identifier: {identifier}")?;
        }
        if let Some(physical_location) = &self.physical_location {
            writeln!(f, "Physical Location of location: {physical_location}")?;
        }
        if let Some(description) = &self.description {
            writeln!(f, "Description: {description}")?;
        }
        Ok(())
    }
}
