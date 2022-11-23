use super::connector_type;
use serde::{Deserialize, Serialize};
use std::fmt;
//TODO: Make some of these fields enums
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct EquipmentConnector {
    pub connector: Option<connector_type::ConnectorType>,
    pub direction: Option<String>,
    pub face: Option<String>,
    pub x: Option<u64>,
    pub y: Option<u64>,
}
impl fmt::Display for EquipmentConnector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Equipment Connector:")?;
        if let Some(foo) = &self.connector {
            write!(f, "Connector: {}", foo)?;
        }
        if let Some(foo) = &self.direction {
            write!(f, "Direction: {}", foo)?;
        }
        if let Some(foo) = &self.face {
            write!(f, "Face: {}", foo)?;
        }
        if let Some(foo) = &self.x {
            write!(f, "X coordinate: {}", foo)?;
        }
        if let Some(foo) = &self.y {
            write!(f, "Y coordinate: {}", foo)?;
        }
        Ok(())
    }
}
