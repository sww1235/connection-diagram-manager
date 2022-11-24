///`cable_type` represents a cable with multiple cores
pub mod cable_type;
/// `connector_type` represents a connector
pub mod connector_type;
/// `equipment_connector represents a connector instance on a piece of equipment
pub mod equipment_connector;
/// `equipment_type` represents a type of equipment
pub mod equipment_type;
/// `location_type` represents a type of location
pub mod location_type;
/// `pathway_type` represents a type of pathway for wires or cables
pub mod pathway_type;
/// `svg` represents a complete SVG image
pub mod svg;
/// `term_cable_type` represents a cable that has connectors assembled on to it
pub mod term_cable_type;
/// `wire_type` represents an individual wire with optional insulation
pub mod wire_type;

/// `equipment` represents an instance of an EquipmentType. This is a physical item
/// you hold in your hand.
pub mod equipment;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs::{self};

/// `Data` represents all data that can be parsed from one source file.
/// the `append` method allows a `Data` struct to be appended onto another one
/// by merging the invididual hashmaps. This allows you to have one main
/// variable in your main function that holds all the data for your program
///
/// The reason all this data has to live in one struct, is to allow the
/// YAML document to be deserialized correctly.
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

impl Data {
    //https://users.rust-lang.org/t/merge-two-structs/68889
    //https://stackoverflow.com/questions/27244465/merge-two-hashmaps-in-rust
    /// `append` merges the inner hashmaps of 2 `Data` structs
    pub fn append(&mut self, other: Data) {}
}
/// `data_parser` deserializes a provided file handle into a Data Struct
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
