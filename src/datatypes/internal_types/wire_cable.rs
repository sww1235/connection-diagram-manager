use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

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
    pub length: Option<f64>,
    /// Pathway containing instance
    pub pathway: Option<Rc<RefCell<Pathway>>>,
}
/// `WireCableType` allows a `WireCable` to store a reference to either a `WireType`, `CableType`
/// or `TermCableType`
#[derive(Debug, PartialEq)]
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
