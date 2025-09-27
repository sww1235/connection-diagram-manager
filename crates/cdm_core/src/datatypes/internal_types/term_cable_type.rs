use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::datatypes::{
    unit_helper::Length,
    util_types::{Catalog, LineStyle},
};

/// `TermCableType` represents a terminated cable with 2 ends and a connector on at least 1 end.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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
    CableType(String),
    /// `WireType`
    WireType(String),
}

/// `TermCableConnectorTermination` represents the connections between a pin of an individual
/// `TermCableConnector` and the individual core of the cable.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Termination {
    /// `Core` represents which individual wire inside a cable this pin is connected to
    pub core: String,
    /// `Pin` represents which pin in the associated connector the core is connected to
    pub pin: String,
}

/// `TermCableConnector` represents a connector on one end of a `TermCable`
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Connector {
    /// `connector_type` represents the connector type that is on the end of a `TermCable`
    pub connector_type: String,
    /// `terminations` represents the pin/core mapping for this connector
    pub terminations: Vec<Termination>,
}

impl connector::Connector for Connector {
    fn pin_count(&self) -> u64 {
        #[expect(clippy::unwrap_used)]
        // allowing unwrap as I want a panic here if this application
        // is used on a 128 bit architecture
        u64::try_from(self.connector_type.borrow().pins.len()).unwrap()
    }
}
