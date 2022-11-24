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
        if let Some(connector) = &self.connector {
            writeln!(f, "Connector: {}", connector)?;
        }
        if let Some(direction) = &self.direction {
            writeln!(f, "Direction: {}", direction)?;
        }
        if let Some(face) = &self.face {
            writeln!(f, "Face: {}", face)?;
        }
        if let Some(x) = &self.x {
            writeln!(f, "X coordinate: {}", x)?;
        }
        if let Some(y) = &self.y {
            writeln!(f, "Y coordinate: {}", y)?;
        }
        Ok(())
    }
}
