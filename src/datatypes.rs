pub mod cable_type;
pub mod connector_type;
pub mod equipment_connector;
pub mod equipment_type;
pub mod location_type;
pub mod pathway_type;
pub mod svg;
pub mod term_cable_type;
pub mod wire_type;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Data {
    #[serde(rename = "wire_type")]
    wire_types: Option<HashMap<String, wire_type::WireType>>,
    #[serde(rename = "cable_type")]
    cable_types: Option<HashMap<String, cable_type::CableType>>,
    #[serde(rename = "term_cable_type")]
    term_cable_types: Option<HashMap<String, term_cable_type::TermCableType>>,
    #[serde(rename = "location_type")]
    location_types: Option<HashMap<String, location_type::LocationType>>,
    #[serde(rename = "connector_type")]
    connector_types: Option<HashMap<String, connector_type::ConnectorType>>,
    #[serde(rename = "equipment_type")]
    equipement_types: Option<HashMap<String, equipment_type::EquipmentType>>,
    #[serde(rename = "pathway_type")]
    pathway_types: Option<HashMap<String, pathway_type::PathwayType>>,
    //TODO: create structs for individual values
}

pub fn data_parser(data_file: fs::File) -> Result<Data, serde_yaml::Error> {
    let data: Data = serde_yaml::from_reader(data_file)?;
    Ok(data)
}
