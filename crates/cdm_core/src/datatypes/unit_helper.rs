use serde::{Deserialize, Serialize};
use uom::si::rational64;

/// Struct representing `Area` values
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[expect(
    clippy::partial_pub_fields,
    missing_docs,
    clippy::missing_docs_in_private_items
)]
pub struct Area {
    pub value: rational64::Area,
    unit_string: String,
}

/// Struct representing `Cross Sectional Area` values of conductors
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[expect(
    clippy::partial_pub_fields,
    missing_docs,
    clippy::missing_docs_in_private_items
)]
pub struct CrossSectionalArea {
    pub value: rational64::Area,
    unit_string: String,
}

/// Struct representing `Electric Potential` values
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[expect(
    clippy::partial_pub_fields,
    missing_docs,
    clippy::missing_docs_in_private_items
)]
pub struct ElectricPotential {
    pub value: rational64::ElectricPotential,
    unit_string: String,
}

/// Struct representing `Length` values
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[expect(
    clippy::partial_pub_fields,
    missing_docs,
    clippy::missing_docs_in_private_items
)]
pub struct Length {
    pub value: rational64::Length,
    unit_string: String,
}

/// Struct representing `TemperatureInterval` values
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[expect(
    clippy::partial_pub_fields,
    missing_docs,
    clippy::missing_docs_in_private_items
)]
pub struct TemperatureInterval {
    pub value: rational64::TemperatureInterval,
    unit_string: String,
}

//TODO: implement to/from CrossSectionalArea to AWG
//
//TODO: implement to/from Area to CrossSectionalArea
