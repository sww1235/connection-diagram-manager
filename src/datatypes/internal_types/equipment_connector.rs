use super::connector_type::ConnectorType;

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
//TODO: Make some of these fields enums
/// EquipmentConnector represents an instance of a [`ConnectorType`](super::connector_type::ConnectorType) in
/// a EquipmentType
#[derive(Debug, Default)]
pub struct EquipmentConnector {
    /// Internal ID of `EquipmentConnector`
    /// ConnectorType
    pub connector: Option<Rc<RefCell<ConnectorType>>>,
    /// electrical direction, used for basic rule mapping, (input, output, power input, power
    /// output, bidirectiona, passive)
    pub direction: Option<String>,
    /// which face the connector is on
    // TODO: refactor this into a face struct
    pub face: Option<String>,
    /// location of connector on face from left of visrep. Origin is bottom left
    pub x: Option<u64>,
    /// location of connector on face from bottom of visrep. Origin is bottom left
    pub y: Option<u64>,
}
impl fmt::Display for EquipmentConnector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Equipment Connector:")?;
        if let Some(connector) = &self.connector {
            writeln!(f, "Connector: {}", connector.borrow())?;
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
