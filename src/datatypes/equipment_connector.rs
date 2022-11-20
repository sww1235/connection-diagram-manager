use super::connector_type;
use serde::{Deserialize, Serialize};
//TODO: Make some of these fields enums
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct EquipmentConnector {
    pub connector: connector_type::ConnectorType,
    pub direction: String,
    pub face: String,
    pub x: u64,
    pub y: u64,
}
