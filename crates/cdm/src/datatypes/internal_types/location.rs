use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use cdm_traits::{Empty, Mergable, PartialEmpty};

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
    #[must_use]
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

impl Mergable for Location {
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
        if self.location_type != other.location_type {
            input_map.insert(
                "Location Type".to_string(),
                [
                    self.location_type.borrow().id.clone(),
                    other.location_type.borrow().id.clone(),
                ],
            );
        }
        if self.identifier != other.identifier {
            input_map.insert(
                "Identifier".to_string(),
                [
                    {
                        if let Some(identifier) = self.identifier.clone() {
                            identifier
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(identifier) = other.identifier.clone() {
                            identifier
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
        if self.physical_location != other.physical_location {
            input_map.insert(
                "Physical Location".to_string(),
                [
                    {
                        if let Some(physical_location) = self.physical_location.clone() {
                            physical_location
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(physical_location) = other.physical_location.clone() {
                            physical_location
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        let results = prompt_fn(input_map);
        // false means don't replace value in self struct
        if results["Location Type"] {
            self.location_type = Rc::clone(&other.location_type);
        }
        if results["Identifier"] {
            self.identifier = other.identifier.clone();
        }
        if results["Description"] {
            self.description = other.description.clone();
        }
        if results["Physical Location"] {
            self.physical_location = other.physical_location.clone();
        }
    }
}

impl Empty for Location {
    fn is_empty(&self) -> bool {
        self == &Self::new()
    }
}

impl PartialEmpty for Location {
    fn is_partial_empty(&self) -> bool {
        let tester = Self::new();
        self.location_type == tester.location_type
            && self.identifier == tester.identifier
            && self.physical_location == tester.physical_location
            && self.description == tester.description
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
