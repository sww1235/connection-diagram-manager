use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use uom::si::rational64::Length;

use cdm_macros::{Empty, Merge, PartialEmpty};
use cdm_traits::{connector, partial_empty::PartialEmpty};

use crate::datatypes::{
    color::Color,
    internal_types::{cable_type::CableType, connector_type::ConnectorType, wire_type::WireType},
    util_types::{Catalog, LineStyle},
};

/// `TermCableType` represents a terminated cable with 2 ends and a connector on at least 1 end.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TermCableType {
    /// Catalog information
    pub catalog: Option<Catalog>,
    /// Underlying wire or cable type of Terminated Cable
    pub wire_cable: WireCable,
    /// Nominal Length of Terminated Cable
    pub nominal_length: Option<Length>,
    nominal_length_unit: Option<String>,
    /// Actual Length of Terminated Cable
    pub actual_length: Option<Length>,
    actual_length_unit: Option<String>,
    /// appearance in schematics
    pub line_style: Option<LineStyle>,
    /// One end of Terminated Cable.
    pub end1: HashMap<String, Connector>,
    /// The other end of Terminated Cable
    pub end2: HashMap<String, Connector>,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}

/// `WireCable` allows either a `WireType` or `CableType` to be the root of a `TermCableType`
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[expect(clippy::exhaustive_enums)]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Termination {
    /// `Core` represents which individual wire inside a cable this pin is connected to
    pub core: String,
    /// `Pin` represents which pin in the associated connector the core is connected to
    pub pin: String,
}

/// `TermCableConnector` represents a connector on one end of a `TermCable`
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
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
        #[expect(clippy::unwrap_used)]
        // allowing unwrap as I want a panic here if this application
        // is used on a 128 bit architecture
        u64::try_from(self.connector_type.borrow().pins.len()).unwrap()
    }
}
