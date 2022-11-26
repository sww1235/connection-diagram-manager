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
use std::fs::{self, File};
use std::io;
use std::path;

use log::trace;

/// `Data` represents all data that can be parsed from one source file.
///
/// The reason all this data has to live in one struct, is to allow the
/// YAML document to be deserialized correctly.
#[derive(Serialize, Deserialize, Debug, Default)]
struct Data {
    /// stores all WireTypes read in from file
    #[serde(rename = "wire_type")]
    wire_types: Option<HashMap<String, wire_type::WireType>>,
    /// stores all CableTypes read in from file
    #[serde(rename = "cable_type")]
    cable_types: Option<HashMap<String, cable_type::CableType>>,
    /// stores all TermCableTypes read in from file
    #[serde(rename = "term_cable_type")]
    term_cable_types: Option<HashMap<String, term_cable_type::TermCableType>>,
    /// stores all LocationTypes read in from file
    #[serde(rename = "location_type")]
    location_types: Option<HashMap<String, location_type::LocationType>>,
    /// stores all ConnectorTypes read in from file
    #[serde(rename = "connector_type")]
    connector_types: Option<HashMap<String, connector_type::ConnectorType>>,
    /// stores all EquipmentTypes read in from file
    #[serde(rename = "equipment_type")]
    equipment_types: Option<HashMap<String, equipment_type::EquipmentType>>,
    /// stores all PathwayTypes read in from file
    #[serde(rename = "pathway_type")]
    pathway_types: Option<HashMap<String, pathway_type::PathwayType>>,
    //TODO: create structs for individual values
}

/// `Datastore` represents all data that is read from all source files
#[derive(Serialize, Debug, Default)]
pub struct Datastore {
    /// contains all
    #[serde(rename = "wire_type")]
    pub wire_types: HashMap<String, wire_type::WireType>,
    #[serde(rename = "cable_type")]
    pub cable_types: HashMap<String, cable_type::CableType>,
    #[serde(rename = "term_cable_type")]
    pub term_cable_types: HashMap<String, term_cable_type::TermCableType>,
    #[serde(rename = "location_type")]
    pub location_types: HashMap<String, location_type::LocationType>,
    #[serde(rename = "connector_type")]
    pub connector_types: HashMap<String, connector_type::ConnectorType>,
    #[serde(rename = "equipment_type")]
    pub equipment_types: HashMap<String, equipment_type::EquipmentType>,
    #[serde(rename = "pathway_type")]
    pub pathway_types: HashMap<String, pathway_type::PathwayType>,
    //TODO: create structs for individual values
}

impl Datastore {
    //https://users.rust-lang.org/t/merge-two-structs/68889
    //https://stackoverflow.com/questions/27244465/merge-two-hashmaps-in-rust
    /// `append` takes a `Data` struct and merges it into a `Datastore` struct
    /// while also de-Optioning it
    fn append(&mut self, other: Data, filepath: Option<path::PathBuf>) {
        // wire_types
        if let Some(wire_types) = other.wire_types {
            for (k, v) in wire_types {
                if self.wire_types.contains_key(&k) {
                    //TODO: do something: ignore dupe, prompt user for merge, try to merge
                    //automatically
                } else {
                    trace! {"Inserted key: {:?}, value: {:?} into main datastore.",k,v}
                    self.wire_types.insert(k, v);
                }
            }
        } // wire_types
        if let Some(cable_types) = other.cable_types {
            for (k, v) in cable_types {
                if self.cable_types.contains_key(&k) {
                    //TODO: do something: ignore dupe, prompt user for merge, try to merge
                    //automatically
                } else {
                    trace! {"Inserted key: {:?}, value: {:?} into main datastore.",k,v}
                    self.cable_types.insert(k, v);
                }
            }
        }
    }

    ///Initializes an empty Datastore
    pub fn new() -> Datastore {
        let datastore = Datastore {
            wire_types: HashMap::new(),
            cable_types: HashMap::new(),
            term_cable_types: HashMap::new(),
            location_types: HashMap::new(),
            connector_types: HashMap::new(),
            equipment_types: HashMap::new(),
            pathway_types: HashMap::new(),
        };
        datastore
    }
}

/// `data_parser` deserializes a provided file handle into a Data Struct
fn data_parser(data_file: fs::File) -> Result<Data, serde_yaml::Error> {
    let data: Data = serde_yaml::from_reader(data_file)?;
    //TODO: validate data and translate to actual types from strings
    Ok(data)
}
/// ParserError wraps serde_yaml::Error and io::Error to allow them both to be returned from one
/// function
pub enum ParserError {
    /// Wrapper for serde_yaml::Error
    YamlError(serde_yaml::Error),
    /// Wrapper for io::Error
    IOError(io::Error),
}

impl From<serde_yaml::Error> for ParserError {
    fn from(err: serde_yaml::Error) -> ParserError {
        ParserError::YamlError(err)
    }
}
impl From<io::Error> for ParserError {
    fn from(err: io::Error) -> ParserError {
        ParserError::IOError(err)
    }
}

/// `project_dir_parser` takes in a project directory and parses all source files found within
pub fn parse_project_dir(project_dir: path::PathBuf) -> Result<Datastore, ParserError> {
    let mut datastore = Datastore::new();
    if project_dir.as_path().is_dir() {
        proj_dir_parse_inner(project_dir, &mut datastore)?;
    } else {
        //TODO: return is not directory error
    }

    Ok(datastore)
}

fn proj_dir_parse_inner(
    inner_dir: path::PathBuf,
    datastore: &mut Datastore,
) -> Result<(), ParserError> {
    for entry in fs::read_dir(inner_dir)? {
        let entry = entry?; // read_dir returns result
        let path = entry.path();

        if path.is_dir() {
            proj_dir_parse_inner(path, datastore)?;
        } else if path.is_file() {
            let data = data_parser(File::open(&path)?)?;
            Datastore::append(datastore, data, Some(path));
        }
    }
    Ok(())
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
