use std::cell::RefCell;
use std::fmt;
use std::path::PathBuf;
use std::rc::Rc;

use dimensioned::ucum;

use cdm_macros::{Empty, Merge, PartialEmpty};

use super::{connector_type::ConnectorType, pathway::Pathway, wire_type::WireType};

/// `Wire` represents a particular instance of a `WireType`.
/// It represents a physical item.
#[derive(Debug, Default, PartialEq, Merge, PartialEmpty, Empty)]
pub struct Wire {
    /// Internal `id` of `Wire` instance
    pub id: String,
    /// The `WireType` of this instance
    pub wire_type: Rc<RefCell<WireType>>,
    /// The structured name of the `Wire` instance
    pub identifier: Option<String>,
    /// Optional description
    pub description: Option<String>,
    /// length of wire
    pub length: ucum::Meter<f64>,
    /// Pathway containing instance
    pub pathway: Option<Rc<RefCell<Pathway>>>,
    /// One end of `Wire` / Cable.
    pub end1: Connector,
    /// The other end of `Wire`
    pub end2: Connector,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}

/// `WireConnector` is a connector on one end of a `Wire`
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Connector {
    /// `connector_type` represents the connector type that is on the end of a `Wire`
    pub connector_type: Rc<RefCell<ConnectorType>>,
}
impl Wire {
    /// Creates an empty instance of `WireCable`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl fmt::Display for Wire {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Wire Instance:")?;
        if let Some(manufacturer) = self.wire_type.borrow().manufacturer.clone() {
            writeln!(f, "Manufacturer: {manufacturer}")?;
        }
        //TODO: Decide how much data from Equiptype we want to display for instance
        if let Some(model) = self.wire_type.borrow().model.clone() {
            writeln!(f, "Model: {model}")?;
        }
        if let Some(part_number) = self.wire_type.borrow().part_number.clone() {
            writeln!(f, "Part Number: {part_number}")?;
        }
        if let Some(manufacturer_part_number) =
            self.wire_type.borrow().manufacturer_part_number.clone()
        {
            writeln!(f, "Manufacturer Part Number: {manufacturer_part_number}")?;
        }
        if let Some(supplier) = self.wire_type.borrow().supplier.clone() {
            writeln!(f, "Supplier: {supplier}")?;
        }
        if let Some(supplier_part_number) = self.wire_type.borrow().supplier_part_number.clone() {
            writeln!(f, "Supplier Part Number: {supplier_part_number}")?;
        }
        writeln!(f, "Length: {}", &self.length)?;
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
