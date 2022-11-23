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
        writeln!(f, "Equipment Connector:")?;
        if let Some(foo) = &self.connector {
            writeln!(f, "Connector: {}", foo)?;
        }
        if let Some(foo) = &self.direction {
            writeln!(f, "Direction: {}", foo)?;
        }
        if let Some(foo) = &self.face {
            writeln!(f, "Face: {}", foo)?;
        }
        if let Some(foo) = &self.x {
            writeln!(f, "X coordinate: {}", foo)?;
        }
        if let Some(foo) = &self.y {
            writeln!(f, "Y coordinate: {}", foo)?;
        }
        Ok(())
    }
}
