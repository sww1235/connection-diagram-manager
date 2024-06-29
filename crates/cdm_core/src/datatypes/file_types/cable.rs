use serde::{Deserialize, Serialize};

/// `Cable` represents a particular instance of a `CableType`
/// It represents a physical item.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Cable {
    /// Internal `id` of `Cable` instance
    pub id: String,
    /// The `CableType` of this instance
    #[serde(rename = "type")]
    pub cable_type: String,
    /// The structured name of the `Cable` instance
    pub identifier: Option<String>,
    /// Optional description
    pub description: Option<String>,
    /// length of cable
    pub length: f64,
    /// Pathway containing instance
    pub pathway: Option<String>,
    /// Connectors on one end of `Cable`
    pub end1: Vec<Connector>,
    /// Connectors on other end of `Cable`
    pub end2: Vec<Connector>,
}
/// `CableConnectorTermination` represents the connections between a pin of an individual
/// `CableConnector` and the individual core of the cable.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Termination {
    /// `Core` represents which individual wire inside a cable this pin is connected to
    pub core: Option<u64>,
    /// `Pin` represents which pin in the associated connector the core is connected to
    pub pin: Option<u64>,
}

/// `WireCableConnector` represents a connector on one end of a `Cable`
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Connector {
    /// `connector_type` represents the connector type that is on the end of a `Cable`
    pub connector_type: String,
    /// `terminations` represents the pin/core mapping for this connector
    pub terminations: Option<Vec<Termination>>,
}
