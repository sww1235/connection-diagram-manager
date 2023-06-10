use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use super::{equipment_type::EquipmentType, location::Location, Empty, Mergable, PartialEmpty};

/// `Equipment` represents a particular instance of an `EquipmentType`.
/// This is the physical unit you would hold in your hand
#[derive(Debug, Default, PartialEq)]
pub struct Equipment {
    /// Internal `id` of equipment instance
    pub id: String,
    /// The type of equipment of the instance
    pub equip_type: Rc<RefCell<EquipmentType>>,
    /// The structured name of the equipment
    pub identifier: Option<String>,
    /// The particular mounting type of this instance
    /// must be in list of mounting types defined in `equip_type.mounting_type`
    pub mounting_type: Option<String>,
    /// The individual location
    pub location: Option<Rc<RefCell<Location>>>,
    /// Description
    pub description: Option<String>,
}
impl Equipment {
    /// Creates an empty instance of `Equipment`
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: String::new(),
            equip_type: Rc::new(RefCell::new(EquipmentType::new())),
            identifier: None,
            mounting_type: None,
            location: None,
            description: None,
        }
    }
}

impl Mergable for Equipment {
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
        if self.equip_type != other.equip_type {
            input_map.insert(
                "Equipment Type".to_string(),
                [
                    self.equip_type.borrow().id.clone(),
                    other.equip_type.borrow().id.clone(),
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
        if self.mounting_type != other.mounting_type {
            input_map.insert(
                "Mounting Type".to_string(),
                [
                    {
                        if let Some(mounting_type) = self.mounting_type.clone() {
                            mounting_type
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(mounting_type) = other.mounting_type.clone() {
                            mounting_type
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.location != other.location {
            input_map.insert(
                "Location".to_string(),
                [
                    {
                        if let Some(location) = self.location.clone() {
                            location.borrow().id.clone()
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(location) = other.location.clone() {
                            location.borrow().id.clone()
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
        let results = prompt_fn(input_map);
        // false means don't replace value in self struct
        if results["Equipment Type"] {
            self.equip_type = Rc::clone(&other.equip_type);
        }
        if results["Identifier"] {
            self.identifier = other.identifier.clone();
        }
        if results["Mounting Type"] {
            self.mounting_type = other.mounting_type.clone();
        }
        if results["Location"] {
            self.location = other.location.clone();
        }
        if results["Description"] {
            self.description = other.description.clone();
        }
    }
}

impl Empty for Equipment {
    fn is_empty(&self) -> bool {
        self == &Self::new()
    }
}

impl PartialEmpty for Equipment {
    fn is_partial_empty(&self) -> bool {
        let tester = Self::new();
        self.equip_type == tester.equip_type
            && self.identifier == tester.identifier
            && self.mounting_type == tester.mounting_type
            && self.location == tester.location
            && self.description == tester.description
    }
}

impl fmt::Display for Equipment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Equipment Instance:")?;
        if let Some(manufacturer) = &self.equip_type.borrow().manufacturer {
            writeln!(f, "Manufacturer: {manufacturer}")?;
        }
        //TODO: Decide how much data from Equiptype we want to display for instance
        if let Some(model) = &self.equip_type.borrow().model {
            writeln!(f, "Model: {model}")?;
        }
        if let Some(part_number) = &self.equip_type.borrow().part_number {
            writeln!(f, "Part Number: {part_number}")?;
        }
        if let Some(manufacturer_part_number) = &self.equip_type.borrow().manufacturer_part_number {
            writeln!(f, "Manufacturer Part Number: {manufacturer_part_number}")?;
        }
        if let Some(supplier) = &self.equip_type.borrow().supplier {
            writeln!(f, "Supplier: {supplier}")?;
        }
        if let Some(supplier_part_number) = &self.equip_type.borrow().supplier_part_number {
            writeln!(f, "Supplier Part Number: {supplier_part_number}")?;
        }
        if let Some(identifier) = &self.identifier {
            writeln!(f, "Equipment Identifier: {identifier}")?;
        }
        if let Some(mounting_type) = &self.mounting_type {
            writeln!(f, "Mounting Type: {mounting_type}")?;
        }
        if let Some(location) = &self.location {
            writeln!(f, "Location: {}", location.borrow())?;
        }
        if let Some(description) = &self.description {
            writeln!(f, "Description: {description}")?;
        }
        Ok(())
    }
}
