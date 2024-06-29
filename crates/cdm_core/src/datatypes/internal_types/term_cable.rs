use std::cell::RefCell;
use std::fmt;
use std::path::PathBuf;
use std::rc::Rc;

use dimensioned::ucum;

use cdm_macros::{Empty, Merge, PartialEmpty};

use super::{pathway::Pathway, term_cable_type::TermCableType};

/// `TermCable` represents a particular instance of a `TermCableType`.
/// It represents a physical item.
#[derive(Debug, Default, PartialEq, Merge, PartialEmpty, Empty)]
pub struct TermCable {
    /// Internal `id` of `TermCable` instance
    pub id: String,
    /// The `TermCableType` of this instance
    pub term_cable_type: Rc<RefCell<TermCableType>>,
    /// The structured name of the `TermCable` instance
    pub identifier: Option<String>,
    /// Optional description
    pub description: Option<String>,
    /// Pathway containing instance
    pub pathway: Option<Rc<RefCell<Pathway>>>,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}
impl TermCable {
    /// Creates an empty instance of `WireCable`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// length of `TermCableType`
    #[must_use]
    pub fn len(&self) -> ucum::Meter<f64> {
        self.term_cable_type.borrow().actual_length.unwrap_or(
            self.term_cable_type
                .borrow()
                .nominal_length
                .unwrap_or_default(),
        )
    }
}

impl fmt::Display for TermCable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Term Cable Instance:")?;
        if let Some(manufacturer) = self.term_cable_type.borrow().manufacturer.clone() {
            writeln!(f, "Manufacturer: {manufacturer}")?;
        }
        //TODO: Decide how much data from Equiptype we want to display for instance
        if let Some(model) = self.term_cable_type.borrow().model.clone() {
            writeln!(f, "Model: {model}")?;
        }
        if let Some(part_number) = self.term_cable_type.borrow().part_number.clone() {
            writeln!(f, "Part Number: {part_number}")?;
        }
        if let Some(manufacturer_part_number) = self
            .term_cable_type
            .borrow()
            .manufacturer_part_number
            .clone()
        {
            writeln!(f, "Manufacturer Part Number: {manufacturer_part_number}")?;
        }
        if let Some(supplier) = self.term_cable_type.borrow().supplier.clone() {
            writeln!(f, "Supplier: {supplier}")?;
        }
        if let Some(supplier_part_number) =
            self.term_cable_type.borrow().supplier_part_number.clone()
        {
            writeln!(f, "Supplier Part Number: {supplier_part_number}")?;
        }
        writeln!(f, "Length: {}", self.len())?;
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
