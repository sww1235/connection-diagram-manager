use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use dimensioned::ucum;

use cdm_traits::{Empty, Mergable, PartialEmpty};

use super::{
    cable_type::CableType, pathway::Pathway, term_cable_type::TermCableType, wire_type::WireType,
};

//TODO: maybe split this up into separate structs

/// `WireCable` represents a particular instance of a `WireType`, `CableType` or `TermCableType`.
/// It represents a physical item.
#[derive(Debug, Default, PartialEq)]
pub struct WireCable {
    /// Internal `id` of wire or cable instance
    pub id: String,
    /// The type of wire, cable or term_cable of this instance
    pub ctw_type: WireCableType,
    /// The structured name of the wire/cable/termcable instance
    pub identifier: Option<String>,
    /// Optional description
    pub description: Option<String>,
    /// length of wire or cable TODO: figure out how to return term_cable_length if it is defined
    pub length: Option<ucum::Meter<f64>>,
    /// Pathway containing instance
    pub pathway: Option<Rc<RefCell<Pathway>>>,
}
/// `WireCableType` allows a `WireCable` to store a reference to either a `WireType`, `CableType`
/// or `TermCableType`
#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
#[allow(clippy::module_name_repetitions)]
pub enum WireCableType {
    /// `CableType`
    CableType(Rc<RefCell<CableType>>),
    /// `TermCableType`
    TermCableType(Rc<RefCell<TermCableType>>),
    /// `WireType`
    WireType(Rc<RefCell<WireType>>),
}

// have to implement default for this for some weird reason
impl Default for WireCableType {
    fn default() -> Self {
        WireCableType::WireType(Rc::new(RefCell::new(WireType::new())))
    }
}
impl WireCable {
    /// Creates an empty instance of `WireCable`
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: String::new(),
            ctw_type: WireCableType::default(),
            identifier: None,
            description: None,
            length: None,
            pathway: None,
        }
    }
}

#[allow(clippy::too_many_lines)]
// TODO: see if this can be split up
impl Mergable for WireCable {
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
        if self.ctw_type != other.ctw_type {
            let self_string = match &self.ctw_type {
                WireCableType::CableType(cable_type) => cable_type.borrow().id.clone(),
                WireCableType::TermCableType(term_cable_type) => {
                    term_cable_type.borrow().id.clone()
                }
                WireCableType::WireType(wire_type) => wire_type.borrow().id.clone(),
            };
            let other_string = match &other.ctw_type {
                WireCableType::CableType(cable_type) => cable_type.borrow().id.clone(),
                WireCableType::TermCableType(term_cable_type) => {
                    term_cable_type.borrow().id.clone()
                }
                WireCableType::WireType(wire_type) => wire_type.borrow().id.clone(),
            };

            input_map.insert("CTW Type".to_string(), [self_string, other_string]);
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
        if self.length != other.length {
            input_map.insert(
                "Length".to_string(),
                [
                    {
                        if let Some(length) = self.length {
                            length.to_string()
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(length) = other.length {
                            length.to_string()
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.pathway != other.pathway {
            input_map.insert(
                "Pathway".to_string(),
                [
                    {
                        if let Some(pathway) = self.pathway.clone() {
                            pathway.borrow().id.clone()
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(pathway) = other.pathway.clone() {
                            pathway.borrow().id.clone()
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        let results = prompt_fn(input_map);
        // false means don't replace value in self struct
        if results["CTW Type"] {
            self.ctw_type = other.ctw_type.clone();
        }
        if results["Identifier"] {
            self.identifier = other.identifier.clone();
        }
        if results["Description"] {
            self.description = other.description.clone();
        }
        if results["Length"] {
            self.length = other.length;
        }
        if results["Pathway"] {
            self.pathway = other.pathway.clone();
        }
    }
}

impl Empty for WireCable {
    fn is_empty(&self) -> bool {
        self == &Self::new()
    }
}

impl PartialEmpty for WireCable {
    fn is_partial_empty(&self) -> bool {
        let tester = Self::new();
        self.ctw_type == tester.ctw_type
            && self.identifier == tester.identifier
            && self.length == tester.length
            && self.description == tester.description
            && self.pathway == tester.pathway
    }
}

impl fmt::Display for WireCable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.ctw_type {
            WireCableType::CableType(cable_type) => {
                writeln!(f, "Cable Instance:")?;
                if let Some(manufacturer) = cable_type.borrow().manufacturer.clone() {
                    writeln!(f, "Manufacturer: {manufacturer}")?;
                }
                //TODO: Decide how much data from Equiptype we want to display for instance
                if let Some(model) = cable_type.borrow().model.clone() {
                    writeln!(f, "Model: {model}")?;
                }
                if let Some(part_number) = cable_type.borrow().part_number.clone() {
                    writeln!(f, "Part Number: {part_number}")?;
                }
                if let Some(manufacturer_part_number) =
                    cable_type.borrow().manufacturer_part_number.clone()
                {
                    writeln!(f, "Manufacturer Part Number: {manufacturer_part_number}")?;
                }
                if let Some(supplier) = cable_type.borrow().supplier.clone() {
                    writeln!(f, "Supplier: {supplier}")?;
                }
                if let Some(supplier_part_number) = cable_type.borrow().supplier_part_number.clone()
                {
                    writeln!(f, "Supplier Part Number: {supplier_part_number}")?;
                }
                if let Some(length) = &self.length {
                    writeln!(f, "Length: {length}")?;
                }
            }
            WireCableType::TermCableType(term_cable_type) => {
                writeln!(f, "TermCable Instance:")?;
                if let Some(manufacturer) = term_cable_type.borrow().manufacturer.clone() {
                    writeln!(f, "Manufacturer: {manufacturer}")?;
                }
                //TODO: Decide how much data from Equiptype we want to display for instance
                if let Some(model) = term_cable_type.borrow().model.clone() {
                    writeln!(f, "Model: {model}")?;
                }
                if let Some(part_number) = term_cable_type.borrow().part_number.clone() {
                    writeln!(f, "Part Number: {part_number}")?;
                }
                if let Some(manufacturer_part_number) =
                    term_cable_type.borrow().manufacturer_part_number.clone()
                {
                    writeln!(f, "Manufacturer Part Number: {manufacturer_part_number}")?;
                }
                if let Some(supplier) = term_cable_type.borrow().supplier.clone() {
                    writeln!(f, "Supplier: {supplier}")?;
                }
                if let Some(supplier_part_number) =
                    term_cable_type.borrow().supplier_part_number.clone()
                {
                    writeln!(f, "Supplier Part Number: {supplier_part_number}")?;
                }
                if let Some(length) = term_cable_type.borrow().actual_length {
                    writeln!(f, "Actual Length: {length}")?;
                }
            }
            WireCableType::WireType(wire_type) => {
                writeln!(f, "Wire Instance:")?;
                if let Some(manufacturer) = wire_type.borrow().manufacturer.clone() {
                    writeln!(f, "Manufacturer: {manufacturer}")?;
                }
                //TODO: Decide how much data from Equiptype we want to display for instance
                if let Some(model) = wire_type.borrow().model.clone() {
                    writeln!(f, "Model: {model}")?;
                }
                if let Some(part_number) = wire_type.borrow().part_number.clone() {
                    writeln!(f, "Part Number: {part_number}")?;
                }
                if let Some(manufacturer_part_number) =
                    wire_type.borrow().manufacturer_part_number.clone()
                {
                    writeln!(f, "Manufacturer Part Number: {manufacturer_part_number}")?;
                }
                if let Some(supplier) = wire_type.borrow().supplier.clone() {
                    writeln!(f, "Supplier: {supplier}")?;
                }
                if let Some(supplier_part_number) = wire_type.borrow().supplier_part_number.clone()
                {
                    writeln!(f, "Supplier Part Number: {supplier_part_number}")?;
                }
                if let Some(length) = &self.length {
                    writeln!(f, "Length: {length}")?;
                }
            }
        }
        if let Some(identifier) = &self.identifier {
            writeln!(f, "Equipment Identifier: {identifier}")?;
        }
        if let Some(pathway) = &self.pathway {
            writeln!(f, "Pathway: {}", pathway.borrow())?;
        }
        if let Some(description) = &self.description {
            writeln!(f, "Description: {description}")?;
        }
        Ok(())
    }
}
