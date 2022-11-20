use super::{cable_type, connector_type, wire_type};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TermCableType {
    pub manufacturer: String,
    pub model: String,
    pub part_number: String,
    pub manufacturer_part_number: String,
    pub supplier: String,
    pub supplier_part_number: String,
    pub cable: cable_type::CableType,
    pub wire: wire_type::WireType,
    pub nominal_length: u64,
    pub actual_length: u64,
    pub end1: Vec<TermCableConnector>,
    pub end2: Vec<TermCableConnector>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TermCableConnectorTermination {
    pub core: u64,
    pub pin: u64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TermCableConnector {
    pub connector_type: connector_type::ConnectorType,
    pub terminations: TermCableConnectorTermination,
}
