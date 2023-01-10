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

use log::{trace, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs::{self, File};
use std::io;
use std::path;

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
    /// contains all wire types read in from file, and/or added in via program logic
    #[serde(rename = "wire_type")]
    pub wire_types: HashMap<String, wire_type::WireType>,
    /// contains all cable types read in from file, and/or added in via program logic
    #[serde(rename = "cable_type")]
    pub cable_types: HashMap<String, cable_type::CableType>,
    /// contains all terminated cable types read in from file, and/or added in via program logic
    #[serde(rename = "term_cable_type")]
    pub term_cable_types: HashMap<String, term_cable_type::TermCableType>,
    /// contains all location types read in from file, and/or added in via program logic
    #[serde(rename = "location_type")]
    pub location_types: HashMap<String, location_type::LocationType>,
    /// contains all connector types read in from file, and/or added in via program logic
    #[serde(rename = "connector_type")]
    pub connector_types: HashMap<String, connector_type::ConnectorType>,
    /// contains all equipment types read in from file, and/or added in via program logic
    #[serde(rename = "equipment_type")]
    pub equipment_types: HashMap<String, equipment_type::EquipmentType>,
    /// contains all pathway types read in from file
    #[serde(rename = "pathway_type")]
    pub pathway_types: HashMap<String, pathway_type::PathwayType>,
    //TODO: create structs for individual values
}

impl Datastore {
    //https://users.rust-lang.org/t/merge-two-structs/68889
    //https://stackoverflow.com/questions/27244465/merge-two-hashmaps-in-rust
    /// `append` takes a `Data` struct and merges it into a `Datastore` struct
    /// while also de-Optioning it
    fn append(&mut self, other: Data, filepath: path::PathBuf) {
        // wire_types
        if let Some(wire_types) = other.wire_types {
            for (k, v) in wire_types {
                if self.wire_types.contains_key(&k) {
                    warn! {"WireType : {} with contents: {:#?} has already been loaded. Found again in file {}. Check this and merge if necessary", k, v, filepath.display()}
                    //TODO: do something: ignore dupe, prompt user for merge, try to merge
                    //automatically
                } else {
                    trace! {"Inserted WireType : {}, value: {:#?} into main datastore.",k,v}
                    self.wire_types.insert(k, v);
                }
            }
        }
        // cable_types
        if let Some(cable_types) = other.cable_types {
            for (k, v) in cable_types {
                if self.cable_types.contains_key(&k) {
                    warn! {"CableType : {} with contents: {:#?} has already been loaded. Found again in file {}. Check this and merge if necessary", k, v, filepath.display()}
                    //TODO: do something: ignore dupe, prompt user for merge, try to merge
                    //automatically
                } else {
                    trace! {"Inserted CableType: {}, value: {:#?} into main datastore.",k,v}
                    self.cable_types.insert(k, v);
                }
            }
        }

        // term_cable_types
        if let Some(term_cable_types) = other.term_cable_types {
            for (k, v) in term_cable_types {
                if self.term_cable_types.contains_key(&k) {
                    warn! {"TermCableType : {} with contents: {:#?} has already been loaded. Found again in file {}. Check this and merge if necessary", k, v, filepath.display()}
                    //TODO: do something: ignore dupe, prompt user for merge, try to merge
                    //automatically
                } else {
                    trace! {"Inserted TermCableType: {}, value: {:#?} into main datastore.",k,v}
                    self.term_cable_types.insert(k, v);
                }
            }
        }

        // location_types
        if let Some(location_types) = other.location_types {
            for (k, v) in location_types {
                if self.location_types.contains_key(&k) {
                    warn! {"LocationType : {} with contents: {:#?} has already been loaded. Found again in file {}. Check this and merge if necessary", k, v, filepath.display()}
                    //TODO: do something: ignore dupe, prompt user for merge, try to merge
                    //automatically
                } else {
                    trace! {"Inserted LocationType: {}, value: {:#?} into main datastore.",k,v}
                    self.location_types.insert(k, v);
                }
            }
        }

        // connector_types
        if let Some(connector_types) = other.connector_types {
            for (k, v) in connector_types {
                if self.connector_types.contains_key(&k) {
                    warn! {"ConnectorType : {} with contents: {:#?} has already been loaded. Found again in file {}. Check this and merge if necessary", k, v, filepath.display()}
                    //TODO: do something: ignore dupe, prompt user for merge, try to merge
                    //automatically
                } else {
                    trace! {"Inserted ConnectorType: {}, value: {:#?} into main datastore.",k,v}
                    self.connector_types.insert(k, v);
                }
            }
        }

        // equipment_types
        if let Some(equipment_types) = other.equipment_types {
            for (k, v) in equipment_types {
                if self.equipment_types.contains_key(&k) {
                    warn! {"EquipmentType : {} with contents: {:#?} has already been loaded. Found again in file {}. Check this and merge if necessary", k, v, filepath.display()}
                    //TODO: do something: ignore dupe, prompt user for merge, try to merge
                    //automatically
                } else {
                    trace! {"Inserted EquipmentType: {}, value: {:#?} into main datastore.",k,v}
                    self.equipment_types.insert(k, v);
                }
            }
        }

        // pathway_types
        if let Some(pathway_types) = other.pathway_types {
            for (k, v) in pathway_types {
                if self.pathway_types.contains_key(&k) {
                    warn! {"PathwayType : {} with contents: {:#?} has already been loaded. Found again in file {}. Check this and merge if necessary", k, v, filepath.display()}
                    //TODO: do something: ignore dupe, prompt user for merge, try to merge
                    //automatically
                } else {
                    trace! {"Inserted PathwayType: {}, value: {:#?} into main datastore.",k,v}
                    self.pathway_types.insert(k, v);
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
#[derive(Debug)]
pub enum ParserError {
    /// Wrapper for serde_yaml::Error
    YamlError(serde_yaml::Error),
    /// Wrapper for io::Error
    IOError(io::Error),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self)?;
        Ok(())
    }
}

//impl From<serde_yaml::Error> for ParserError {
//    fn from(err: serde_yaml::Error) -> ParserError {
//        trace! {"{}", err}
//        ParserError::YamlError(err)
//    }
//}
//impl From<io::Error> for ParserError {
//    fn from(err: io::Error) -> ParserError {
//        trace! {"{}", err}
//        ParserError::IOError(err)
//    }
//}

/// `project_dir_parser` takes in a project directory and parses all source files found within
pub fn parse_project_dir(project_dir: path::PathBuf) -> Result<Datastore, io::Error> {
    let mut datastore = Datastore::new();
    if project_dir.as_path().is_dir() {
        proj_dir_parse_inner(project_dir, &mut datastore)?;
        Ok(datastore)
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
    datastore: &mut Datastore,
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
        datastore.append(data, inner_dir);
    } else if inner_dir.is_dir() {
        for entry in fs::read_dir(&inner_dir)? {
            let entry = entry?; // read_dir returns result
            let path = entry.path();
            trace! {"path of entry in inner_dir: {}", path.display()}
            trace! {"{}", inner_dir.display()}
            proj_dir_parse_inner(path, datastore)?;
        }
    } else {
        trace! {"path at else: {}", inner_dir.display()}
        return Ok(());
        //panic! {"this shouldn't ever happen"}
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
