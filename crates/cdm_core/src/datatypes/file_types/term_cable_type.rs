use serde::{Deserialize, Serialize};

/// `TermCableType` represents a terminated cable with 2 ends and a connector on at least 1 end.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TermCableType {
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
    /// Underlying cable type of Terminated Cable
    pub cable: Option<String>,
    /// Underlying wire type of Terminated Cable
    pub wire: Option<String>,
    /// Nominal Length of Terminated Cable
    pub nominal_length: Option<f64>,
    /// Actual Length of Terminated Cable
    pub actual_length: Option<f64>,
    /// One end of Terminated Cable.
    pub end1: Vec<TermCableConnector>,
    /// The other end of Terminated Cable
    pub end2: Vec<TermCableConnector>,
}

/// `TermCableConnectorTermination` represents the connections between a pin of an individual
/// `TermCableConnector` and the individual core of the cable.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TermCableConnectorTermination {
    /// `Core` represents which individual wire inside a cable this pin is connected to
    pub core: Option<u64>,
    /// `Pin` represents which pin in the associated connector the core is connected to
    pub pin: Option<u64>,
}

/// `TermCableConnector` represents a connector on one end of a `TermCable`
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TermCableConnector {
    /// `connector_type` represents the connector type that is on the end of a `TermCable`
    #[serde(rename = "type")]
    pub connector_type: String,
    /// `terminations` represents the pin/core mapping for this connector
    pub terminations: Option<Vec<TermCableConnectorTermination>>,
}
