use std::cell::RefCell;
use std::fmt;
use std::path::PathBuf;
use std::rc::Rc;

use dimensioned::ucum;

use cdm_macros::Merge;
use cdm_traits::{empty::Empty, partial_empty::PartialEmpty};

use super::{cable_type::CableType, connector_type::ConnectorType, wire_type::WireType};

/// `TermCableType` represents a terminated cable with 2 ends and a connector on at least 1 end.
#[derive(Debug, Default, PartialEq, Merge)]
pub struct TermCableType {
    /// Internal ID of `TermCableType`
    pub id: String,
    /// Manufacturer of Terminated cable
    pub manufacturer: Option<String>,
    /// Model of Terminated Cable
    pub model: Option<String>,
    /// Part Number of Terminated Cable
    pub part_number: Option<String>,
    /// Manufacturers part number of Terminated Cable
    pub manufacturer_part_number: Option<String>,
    /// Supplier of Terminated Cable
    pub supplier: Option<String>,
    /// Supplier part number of Terminated Cable
    pub supplier_part_number: Option<String>,
    /// Optional text description of Terminated Cable
    pub description: Option<String>,
    /// Underlying wire or cable type of Terminated Cable
    pub wire_cable: WireCable,
    /// Nominal Length of Terminated Cable
    pub nominal_length: Option<ucum::Meter<f64>>, //TODO: decide if one of these should be optional or not
    /// Actual Length of Terminated Cable
    pub actual_length: Option<ucum::Meter<f64>>,
    /// One end of Terminated Cable.
    pub end1: Vec<TermCableConnector>,
    /// The other end of Terminated Cable
    pub end2: Vec<TermCableConnector>,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}

/// `WireCable` allows either a `WireType` or `CableType` to be the root of a `TermCableType`
#[derive(Debug, PartialEq, Clone)]
#[allow(clippy::exhaustive_enums)]
pub enum WireCable {
    /// CableType
    CableType(Rc<RefCell<CableType>>),
    /// WireType
    WireType(Rc<RefCell<WireType>>),
}

// have to implement default for this for some weird reason
impl Default for WireCable {
    fn default() -> Self {
        WireCable::WireType(Rc::new(RefCell::new(WireType::new())))
    }
}

/// `TermCableConnectorTermination` represents the connections between a pin of an individual
/// `TermCableConnector` and the individual core of the cable.
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TermCableConnectorTermination {
    /// `Core` represents which individual wire inside a cable this pin is connected to
    pub core: Option<u64>,
    /// `Pin` represents which pin in the associated connector the core is connected to
    pub pin: Option<u64>,
}

/// `TermCableConnector` represents a connector on one end of a `TermCable`
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TermCableConnector {
    /// `connector_type` represents the connector type that is on the end of a `TermCable`
    pub connector_type: Rc<RefCell<ConnectorType>>,
    /// `terminations` represents the pin/core mapping for this connector
    pub terminations: Option<Vec<TermCableConnectorTermination>>,
}
impl TermCableType {
    /// Creates an empty instance of `TermCableType`
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: String::new(),
            manufacturer: None,
            model: None,
            part_number: None,
            manufacturer_part_number: None,
            supplier: None,
            supplier_part_number: None,
            description: None,
            wire_cable: WireCable::default(),
            nominal_length: None,
            actual_length: None,
            end1: Vec::new(),
            end2: Vec::new(),
        }
    }
}

impl Empty for TermCableType {
    fn is_empty(&self) -> bool {
        self == &Self::new()
    }
}

impl PartialEmpty for TermCableType {
    fn is_partial_empty(&self) -> bool {
        let tester = Self::new();
        self.manufacturer == tester.manufacturer
            && self.model == tester.model
            && self.part_number == tester.part_number
            && self.manufacturer_part_number == tester.manufacturer_part_number
            && self.supplier == tester.supplier
            && self.supplier_part_number == tester.supplier_part_number
            && self.description == tester.description
            && self.wire_cable == tester.wire_cable
            && self.nominal_length == tester.nominal_length
            && self.actual_length == tester.actual_length
            && self.end1 == tester.end1
            && self.end2 == tester.end2
    }
}

impl fmt::Display for TermCableType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Connector Type:")?;
        if let Some(manufacturer) = &self.manufacturer {
            write!(f, "Manufacturer: {manufacturer}")?;
        }
        if let Some(model) = &self.model {
            write!(f, "Model: {model}")?;
        }
        if let Some(part_number) = &self.part_number {
            write!(f, "Part Number: {part_number}")?;
        }
        if let Some(manufacturer_part_number) = &self.manufacturer_part_number {
            write!(f, "Manufacturer Part Number: {manufacturer_part_number}")?;
        }
        if let Some(supplier) = &self.supplier {
            write!(f, "Supplier: {supplier}")?;
        }
        if let Some(supplier_part_number) = &self.supplier_part_number {
            write!(f, "Supplier Part Number: {supplier_part_number}")?;
        }
        if let Some(description) = &self.description {
            write!(f, "Description: {description}")?;
        }
        match &self.wire_cable {
            WireCable::CableType(cable) => write!(f, "Cable Type: {}", cable.borrow())?,
            WireCable::WireType(wire) => write!(f, "Wire Type: {}", wire.borrow())?,
        }
        if let Some(nominal_length) = &self.nominal_length {
            //TODO: implement units functions to do proper conversions
            write!(f, "Nominal Length: {nominal_length}mm")?;
        }
        if let Some(actual_length) = &self.actual_length {
            //TODO: implement units functions to do proper conversions
            write!(f, "Actual Length: {actual_length} mm")?;
        }
        //TODO: implement loops for cable ends.
        Ok(())
    }
}
