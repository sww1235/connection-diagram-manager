use std::cell::RefCell;
use std::fmt;
use std::path::PathBuf;
use std::rc::Rc;

use dimensioned::ucum;

use cdm_macros::{Empty, Merge, PartialEmpty};

use cdm_traits::partial_empty::PartialEmpty;

use super::{cable_type::CableType, connector::Connector, pathway::Pathway};

/// `Cable` represents a particular instance of a `CableType`
/// It represents a physical item.
#[derive(Debug, Default, PartialEq, Clone, Merge, PartialEmpty, Empty)]
pub struct Cable {
    /// Internal `id` of `Cable` instance
    pub id: String,
    /// The `CableType` of this instance
    pub cable_type: Rc<RefCell<CableType>>,
    /// The structured name of the `Cable` instance
    pub identifier: Option<String>,
    /// Optional description
    pub description: Option<String>,
    /// length of wire or cable
    pub length: ucum::Meter<f64>,
    /// Pathway containing instance
    pub pathway: Option<Rc<RefCell<Pathway>>>,
    /// One end of `Cable`.
    pub end1: Vec<ConnectorJoin>,
    /// The other end of `Cable`.
    pub end2: Vec<ConnectorJoin>,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}

/// `Connector` is a connector on one end of a `Cable`
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ConnectorJoin {
    /// `connector` represents the connector type that is on the end of a `Cable`
    pub connector: Rc<RefCell<Connector>>,
    /// `terminations` represents the pin/core mapping for this connector
    pub terminations: Option<Vec<Termination>>,
}
/// `ConnectorTermination` represents the connections between a pin of an individual
/// `Connector` and the individual core of the cable.
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Termination {
    /// `Core` represents which individual wire inside a cable this pin is connected to
    pub core: u64,
    /// `Pin` represents which pin in the associated connector the core is connected to
    pub pin: u64, //TODO: may want to be able to designate alphanumerically
}

impl Cable {
    /// Creates an empty instance of `Cable`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl fmt::Display for Cable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Cable Instance:")?;
        if let Some(manufacturer) = self.cable_type.borrow().manufacturer.clone() {
            writeln!(f, "Manufacturer: {manufacturer}")?;
        }
        //TODO: Decide how much data from Equiptype we want to display for instance
        if let Some(model) = self.cable_type.borrow().model.clone() {
            writeln!(f, "Model: {model}")?;
        }
        if let Some(part_number) = self.cable_type.borrow().part_number.clone() {
            writeln!(f, "Part Number: {part_number}")?;
        }
        if let Some(manufacturer_part_number) =
            self.cable_type.borrow().manufacturer_part_number.clone()
        {
            writeln!(f, "Manufacturer Part Number: {manufacturer_part_number}")?;
        }
        if let Some(supplier) = self.cable_type.borrow().supplier.clone() {
            writeln!(f, "Supplier: {supplier}")?;
        }
        if let Some(supplier_part_number) = self.cable_type.borrow().supplier_part_number.clone() {
            writeln!(f, "Supplier Part Number: {supplier_part_number}")?;
        }
        writeln!(f, "Length: {}", self.length)?;

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
