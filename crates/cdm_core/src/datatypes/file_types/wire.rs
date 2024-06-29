use serde::{Deserialize, Serialize};

/// `Wire` represents a particular instance of a `WireType`
/// It represents a physical item.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Wire {
    /// Internal `id` of `Wire`
    pub id: String,
    /// The `WireType` of this instance
    #[serde(rename = "type")]
    pub wire_type: String,
    /// The structured name of the `wire` instance
    pub identifier: Option<String>,
    /// Optional description
    pub description: Option<String>,
    /// length of `Wire`
    pub length: f64,
    /// Pathway containing instance
    pub pathway: Option<String>,
    /// Connectors on one end of `Wire`
    pub end1: Connector,
    /// Connectors on other end of `Wire`
    pub end2: Connector,
}

//TODO: somehow validate that this connector only has one pin
/// `WireConnector` represents a connector on one end of a `Wire`
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct Connector {
    /// `connector_type` represents the connector type that is on the end of a `Wire`
    pub connector_type: String,
}
