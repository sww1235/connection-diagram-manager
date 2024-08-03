use std::cell::RefCell;
use std::fmt;
use std::path::PathBuf;
use std::rc::Rc;

use dimensioned::ucum;

use cdm_macros::{Empty, Merge, PartialEmpty};
use cdm_traits::connector;

use super::{cable_type::CableType, connector_type::ConnectorType, wire_type::WireType};

/// `TermCableType` represents a terminated cable with 2 ends and a connector on at least 1 end.
#[derive(Debug, Default, PartialEq, Merge, PartialEmpty, Empty)]
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
    pub end1: Vec<Connector>,
    /// The other end of Terminated Cable
    pub end2: Vec<Connector>,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}

/// `WireCable` allows either a `WireType` or `CableType` to be the root of a `TermCableType`
#[derive(Debug, PartialEq, Clone)]
#[allow(clippy::exhaustive_enums)]
pub enum WireCable {
    /// `CableType`
    CableType(Rc<RefCell<CableType>>),
    /// `WireType`
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
pub struct Termination {
    /// `Core` represents which individual wire inside a cable this pin is connected to
    pub core: u64,
    /// `Pin` represents which pin in the associated connector the core is connected to
    pub pin: u64,
}

/// `TermCableConnector` represents a connector on one end of a `TermCable`
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Connector {
    /// `connector_type` represents the connector type that is on the end of a `TermCable`
    pub connector_type: Rc<RefCell<ConnectorType>>,
    /// `terminations` represents the pin/core mapping for this connector
    pub terminations: Vec<Termination>,
}
impl TermCableType {
    /// Creates an empty instance of `TermCableType`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl connector::Connector for Connector {
    fn pin_count(&self) -> u64 {
        #[allow(clippy::unwrap_used)]
        // allowing unwrap as I want a panic here if this application
        // is used on a 128 bit architecture
        u64::try_from(self.connector_type.borrow().pins.len()).unwrap()
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
