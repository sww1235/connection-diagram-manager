///`cable_type` represents a cable with multiple cores
pub mod cable_type;
/// `connector_type` represents a connector
pub mod connector_type;
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

/// `cable` represents an instance of a `CableType`
pub mod cable;
/// `equipment` represents an instance of an `EquipmentType`. This is a physical item
/// you hold in your hand.
pub mod equipment;
/// `location` represents an instance of a `LocationType`
pub mod location;
/// `pathway` represents an instance of a `PathwayType`
pub mod pathway;
/// `term_cable` represents an instance of a `TermCableType`
pub mod term_cable;
/// `wire` represents an instance of a `WireType`
pub mod wire;

use log::trace;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io;
use std::path;

/// `DataFile` represents all data that can be parsed from one source file.
///
/// The reason all this data has to live in one struct, is to allow the
/// YAML document to be deserialized correctly.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DataFile {
    /// the filepath of the file this data was read in from
    #[serde(skip)]
    pub file_path: path::PathBuf,
    /// stores all `WireTypes` read in from file
    #[serde(rename = "wire_type")]
    pub wire_types: Option<HashMap<String, wire_type::WireType>>,
    /// stores all `CableTypes` read in from file
    #[serde(rename = "cable_type")]
    pub cable_types: Option<HashMap<String, cable_type::CableType>>,
    /// stores all `TermCableTypes` read in from file
    #[serde(rename = "term_cable_type")]
    pub term_cable_types: Option<HashMap<String, term_cable_type::TermCableType>>,
    /// stores all `LocationTypes` read in from file
    #[serde(rename = "location_type")]
    pub location_types: Option<HashMap<String, location_type::LocationType>>,
    /// stores all `ConnectorTypes` read in from file
    #[serde(rename = "connector_type")]
    pub connector_types: Option<HashMap<String, connector_type::ConnectorType>>,
    /// stores all `EquipmentTypes` read in from file
    #[serde(rename = "equipment_type")]
    pub equipment_types: Option<HashMap<String, equipment_type::EquipmentType>>,
    /// stores all `PathwayTypes` read in from file
    #[serde(rename = "pathway_type")]
    pub pathway_types: Option<HashMap<String, pathway_type::PathwayType>>,
    /// stores all `Wires` read in from file
    #[serde(rename = "wire")]
    pub wires: Option<HashMap<String, wire::Wire>>,
    /// stores all `Cables` read in from file
    #[serde(rename = "cable")]
    pub cables: Option<HashMap<String, cable::Cable>>,
    /// stores all `TermCables` read in from file
    #[serde(rename = "term_cable")]
    pub term_cables: Option<HashMap<String, term_cable::TermCable>>,
    /// stores all `Locations` read in from file
    #[serde(rename = "location")]
    pub locations: Option<HashMap<String, location::Location>>,
    /// stores all `Equipment` instances read in from file
    #[serde(rename = "equipment")]
    pub equipment: Option<HashMap<String, equipment::Equipment>>,
    /// stores all `Pathway` instances read in from file
    #[serde(rename = "pathways")]
    pub pathways: Option<HashMap<String, pathway::Pathway>>,
}

/// `data_parser` deserializes a provided file handle into a `DataFile`
fn data_parser(data_file: fs::File) -> Result<DataFile, serde_yaml::Error> {
    let data: DataFile = serde_yaml::from_reader(data_file)?;
    Ok(data)
}

/// `project_dir_parser` takes in a project directory and parses all source files found within
///
/// # Errors
///
/// Will error if:
/// - reading any of the individual project files fails
/// - the specified `project_dir` is not a directory
/// - [`os_str`] failed to parse to UTF-8
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
        //waiting on https://github.com/rust-lang/rust/issues/86442 or
        //https://github.com/rust-lang/rust/pull/128316 to be merged
    }
}

/// `proj_dir_parse_inner` is the recursive function to parse all project directories
fn proj_dir_parse_inner(
    inner_dir: path::PathBuf,
    datafiles: &mut Vec<DataFile>,
) -> Result<(), io::Error> {
    let ext = match inner_dir.extension() {
        Some(ext) => match ext.to_str() {
            Some(ext) => ext,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "os_str failed to parse to valid utf-8",
                ))
            }
        },
        None => "",
    };
    if inner_dir.is_file() && (ext == "yaml" || ext == "yml") {
        trace! {"path at is_file: {}", inner_dir.display()}
        let file_handle = File::open(&inner_dir)?;
        let mut data = match data_parser(file_handle) {
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
        // not a directory or file (maybe a symlink or something?
        return Ok(());
    }

    Ok(())
}
