use super::{cable_type, connector_type, wire_type};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TermCableType {
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub part_number: Option<String>,
    pub manufacturer_part_number: Option<String>,
    pub supplier: Option<String>,
    pub supplier_part_number: Option<String>,
    #[serde(rename = "cable_type")]
    pub cable: Option<cable_type::CableType>,
    #[serde(rename = "wire_type")]
    pub wire: Option<wire_type::WireType>,
    pub nominal_length: Option<u64>,
    pub actual_length: Option<u64>,
    #[serde(rename = "term_cable_connector")]
    pub end1: Option<Vec<TermCableConnector>>,
    #[serde(rename = "term_cable_connector")]
    pub end2: Option<Vec<TermCableConnector>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TermCableConnectorTermination {
    pub core: Option<u64>,
    pub pin: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TermCableConnector {
    pub connector_type: Option<connector_type::ConnectorType>,
    pub terminations: Option<TermCableConnectorTermination>,
}
