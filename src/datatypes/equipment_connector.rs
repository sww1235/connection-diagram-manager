use super::connector_type;
use serde::{Deserialize, Serialize};
//TODO: Make some of these fields enums
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct EquipmentConnector {
    pub connector: Option<connector_type::ConnectorType>,
    pub direction: Option<String>,
    pub face: Option<String>,
    pub x: Option<u64>,
    pub y: Option<u64>,
}
