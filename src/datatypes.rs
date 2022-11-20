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
    wire_types: Option<HashMap<String, wire_type::WireType>>,
    cable_types: Option<HashMap<String, cable_type::CableType>>,
    term_cable_types: Option<HashMap<String, term_cable_type::TermCableType>>,
    location_types: Option<HashMap<String, location_type::LocationType>>,
    connector_types: Option<HashMap<String, connector_type::ConnectorType>>,
    equipement_types: Option<HashMap<String, equipment_type::EquipmentType>>,
    pathway_types: Option<HashMap<String, pathway_type::PathwayType>>,
    //TODO: create structs for individual values
}

pub fn data_parser(data_file: fs::File) {}
