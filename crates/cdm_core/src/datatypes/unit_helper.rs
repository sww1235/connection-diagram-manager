use serde::{Deserialize, Serialize};
use uom::si::rational64;

/// Struct representing `Area` values
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Area {
    /// contained uom Unit
    pub value: rational64::Area,
    /// original unit in datafile
    pub original_unit: String,
}

/// Struct representing `Cross Sectional Area` values of conductors
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CrossSectionalArea {
    /// contained uom Unit
    pub value: rational64::Area,
    /// original unit in datafile
    pub original_unit: String,
}

/// Struct representing `Electric Potential` values
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ElectricPotential {
    /// contained uom Unit
    pub value: rational64::ElectricPotential,
    /// original unit in datafile
    pub original_unit: String,
}

/// Struct representing `Length` values
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Length {
    /// contained uom Unit
    pub value: rational64::Length,
    /// original unit in datafile
    pub original_unit: String,
}

/// Struct representing `TemperatureInterval` values
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TemperatureInterval {
    /// contained uom Unit
    pub value: rational64::TemperatureInterval,
    /// original unit in datafile
    pub original_unit: String,
}

//TODO: implement to/from CrossSectionalArea to AWG
//
//TODO: implement to/from Area to CrossSectionalArea
