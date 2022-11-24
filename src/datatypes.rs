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
use std::fmt;
use std::fs::{self};

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
    equipment_types: Option<HashMap<String, equipment_type::EquipmentType>>,
    #[serde(rename = "pathway_type")]
    pathway_types: Option<HashMap<String, pathway_type::PathwayType>>,
    //TODO: create structs for individual values
}

pub fn data_parser(data_file: fs::File) -> Result<Data, serde_yaml::Error> {
    let data: Data = serde_yaml::from_reader(data_file)?;
    Ok(data)
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(wire_types) = &self.wire_types {
            for (k, v) in wire_types {
                writeln!(f, "Wire Type: {}", k)?;
                writeln!(f, "{}", v)?;
            }
        }
        if let Some(cable_types) = &self.cable_types {
            for (k, v) in cable_types {
                writeln!(f, "Cable Type: {}", k)?;
                writeln!(f, "{}", v)?;
            }
        }
        if let Some(term_cable_types) = &self.term_cable_types {
            for (k, v) in term_cable_types {
                writeln!(f, "Term Cable Type: {}", k)?;
                writeln!(f, "{}", v)?;
            }
        }
        if let Some(location_types) = &self.location_types {
            for (k, v) in location_types {
                writeln!(f, "Location Type: {}", k)?;
                writeln!(f, "{}", v)?;
            }
        }
        if let Some(connector_types) = &self.connector_types {
            for (k, v) in connector_types {
                writeln!(f, "Connector Type: {}", k)?;
                writeln!(f, "{}", v)?;
            }
        }
        if let Some(equipment_types) = &self.equipment_types {
            for (k, v) in equipment_types {
                writeln!(f, "Equipment Type: {}", k)?;
                writeln!(f, "{}", v)?;
            }
        }
        if let Some(pathway_types) = &self.pathway_types {
            for (k, v) in pathway_types {
                writeln!(f, "Pathway Type: {}", k)?;
                writeln!(f, "{}", v)?;
            }
        }
        Ok(())
    }
}
