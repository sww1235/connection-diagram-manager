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

use log::trace;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs::{self, File};
use std::io;
use std::path;

/// `DataFile` represents all data that can be parsed from one source file.
///
/// The reason all this data has to live in one struct, is to allow the
/// YAML document to be deserialized correctly.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DataFile {
    /// the filepath of the file this data was read in from
    #[serde(skip)]
    pub file_path: path::PathBuf,
    /// stores all WireTypes read in from file
    #[serde(rename = "wire_type")]
    pub wire_types: Option<HashMap<String, wire_type::WireType>>,
    /// stores all CableTypes read in from file
    #[serde(rename = "cable_type")]
    pub cable_types: Option<HashMap<String, cable_type::CableType>>,
    /// stores all TermCableTypes read in from file
    #[serde(rename = "term_cable_type")]
    pub term_cable_types: Option<HashMap<String, term_cable_type::TermCableType>>,
    /// stores all LocationTypes read in from file
    #[serde(rename = "location_type")]
    pub location_types: Option<HashMap<String, location_type::LocationType>>,
    /// stores all ConnectorTypes read in from file
    #[serde(rename = "connector_type")]
    pub connector_types: Option<HashMap<String, connector_type::ConnectorType>>,
    /// stores all EquipmentTypes read in from file
    #[serde(rename = "equipment_type")]
    pub equipment_types: Option<HashMap<String, equipment_type::EquipmentType>>,
    /// stores all PathwayTypes read in from file
    #[serde(rename = "pathway_type")]
    pub pathway_types: Option<HashMap<String, pathway_type::PathwayType>>,
    //TODO: create structs for individual values
}

/// `data_parser` deserializes a provided file handle into a Data Struct
fn data_parser(data_file: fs::File) -> Result<DataFile, serde_yaml::Error> {
    let data: DataFile = serde_yaml::from_reader(data_file)?;
    Ok(data)
}

/// `project_dir_parser` takes in a project directory and parses all source files found within
pub fn parse_project_dir(project_dir: path::PathBuf) -> Result<Vec<DataFile>, io::Error> {
    let mut files = Vec::<DataFile>::new();
    if project_dir.as_path().is_dir() {
        proj_dir_parse_inner(project_dir, &mut files)?;
        Ok(files)
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format! {"Provided filepath not a directory {}", project_dir.display()},
        ))
        //TODO: return is not directory error
    }
}

fn proj_dir_parse_inner(
    inner_dir: path::PathBuf,
    datafiles: &mut Vec<DataFile>,
) -> Result<(), io::Error> {
    let ext = match inner_dir.extension() {
        Some(ext) => match ext.to_str() {
            Some(ext) => ext,
            None => panic! {"os_str failed to parse to valid utf-8"},
        },
        None => "",
    };
    if inner_dir.is_file() && (ext == "yaml" || ext == "yml") {
        trace! {"path at is_file: {}", inner_dir.display()}
        let file_handle = File::open(&inner_dir)?;
        let data = match data_parser(file_handle) {
            Ok(data) => data,
            Err(error) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format! {"Parsing yaml file {} failed: {}", inner_dir.display(), error},
                ))
            }
        };
        data.file_path = inner_dir;
        datafiles.push(data);
    } else if inner_dir.is_dir() {
        for entry in fs::read_dir(&inner_dir)? {
            let entry = entry?; // read_dir returns result
            let path = entry.path();
            trace! {"path of entry in inner_dir: {}", path.display()}
            trace! {"{}", inner_dir.display()}
            proj_dir_parse_inner(path, datafiles)?;
        }
    } else {
        trace! {"path at else: {}", inner_dir.display()}
        return Ok(());
        //panic! {"this shouldn't ever happen"}
    }

    Ok(())
}

impl fmt::Display for DataFile {
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
