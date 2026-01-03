/// `cable` represents an instance of a `CableType`
pub mod cable;
/// `connector` represents an instance of a `ConnectorType`
pub mod connector;
/// `enclosure` represents an instance of a `EnclosureType`
pub mod enclosure;
/// `equipment` represents an instance of an `EquipmentType`. This is a physical item
/// you hold in your hand.
pub mod equipment;
/// `mounting_rail` represents an instance of a `MountingRailType`
pub mod mounting_rail;
/// `pathway` represents an instance of a `PathwayType`
pub mod pathway;
/// `term_cable` represents an instance of a `TermCableType`
pub mod term_cable;
/// `terminal_strip` contains the main `terminal_strip` type and accessory types
pub mod terminal_strip;
/// `wire` represents an instance of a `WireType`
pub mod wire;
// TODO: improve this documentation
/// `connection` represents a connection between two different elements
pub mod connection;

use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{
    datatypes::{library_types::Library, project_types::terminal_strip::TermAccy},
    error::{Error, LibraryError, ProjectError},
    traits::FromFile as _,
    util_functions,
};

/// `Project` represents all project specific data used in program
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Project {
    /// contains all cables read in from files, and/or added in via program logic
    pub cables: BTreeMap<String, cable::Cable>,
    /// `connections` contains all connections between different equipment/cables/wires
    pub connections: Vec<connection::Connection>,
    //TODO: are connectors going to be read in separately?
    /// contains all connectors read in from files, and/or added in via program logic
    pub connectors: BTreeMap<String, connector::Connector>,
    /// contains all enclosures read in from files, and/or added in via program logic
    pub enclosures: BTreeMap<String, enclosure::Enclosure>,
    /// contains all equipment read in from files, and/or added in via program logic
    pub equipment: BTreeMap<String, equipment::Equipment>,
    /// contains all mounting rails read in from files, and/or added in via program logic
    pub mounting_rails: BTreeMap<String, mounting_rail::MountingRail>,
    /// contains all pathways read in from files and/or added in via program logic
    pub pathways: BTreeMap<String, pathway::Pathway>,
    /// contains all term cables read in from files, and/or added in via program logic
    pub term_cables: BTreeMap<String, term_cable::TermCable>,
    /// contains all terminal strips read in from files and/or added in via program logic
    pub terminal_strips: BTreeMap<String, terminal_strip::TerminalStrip>,
    /// `wires` contains all wires read in from files, and/or added in via program logic
    pub wires: BTreeMap<String, wire::Wire>,
}

impl Project {
    /// Merges two instances of `Project`, validating that there are no key conflicts between the
    /// two instances
    ///
    /// # Errors
    ///
    /// Will error if there are duplicate keys found in `other` map
    #[inline(never)]
    #[expect(clippy::result_large_err, reason = "Don't want to have to split up error::Error ")]
    pub fn merge(&mut self, test_map: Project, test_file: &Path) -> Result<(), Error> {
        util_functions::merge_btreemaps(&mut self.cables, test_map.cables, test_file)?;
        self.connections.extend(test_map.connections);
        //TODO: are connectors going to be read in separately?
        //util_functions::merge_btreemaps(&mut self.connectors, test_map.connectors, test_file)?;
        util_functions::merge_btreemaps(&mut self.enclosures, test_map.enclosures, test_file)?;
        util_functions::merge_btreemaps(&mut self.equipment, test_map.equipment, test_file)?;
        util_functions::merge_btreemaps(&mut self.mounting_rails, test_map.mounting_rails, test_file)?;
        util_functions::merge_btreemaps(&mut self.pathways, test_map.pathways, test_file)?;
        util_functions::merge_btreemaps(&mut self.term_cables, test_map.term_cables, test_file)?;
        util_functions::merge_btreemaps(&mut self.terminal_strips, test_map.terminal_strips, test_file)?;
        util_functions::merge_btreemaps(&mut self.wires, test_map.wires, test_file)?;
        Ok(())
    }
    /// Inserts datafile path into all structs in the called project
    #[inline(never)]
    pub fn add_datafile_paths(&mut self, datafile_path: &Path) {
        // Cables
        if !self.cables.is_empty() {
            for cable in self.cables.values_mut() {
                cable.set_datafile(datafile_path);
            }
        }
        //TODO: finish this
        // Connections
        //if !self.connections.is_empty() {
        //    for connection in self.connections.values_mut() {
        //        connection.set_datafile(datafile_path);
        //    }
        //}
        // Connectors
        //if !self.connectors.is_empty() {
        //    for connector in self.connectors.values_mut() {
        //        connector.set_datafile(datafile_path);
        //    }
        //}
        // Enclosures
        if !self.enclosures.is_empty() {
            for enclosure in self.enclosures.values_mut() {
                enclosure.set_datafile(datafile_path);
            }
        }
        // Equipment
        if !self.equipment.is_empty() {
            for equipment in self.equipment.values_mut() {
                equipment.set_datafile(datafile_path);
            }
        }
        // Mounting Rails
        if !self.mounting_rails.is_empty() {
            for mounting_rail in self.mounting_rails.values_mut() {
                mounting_rail.set_datafile(datafile_path);
            }
        }
        // Pathways
        if !self.pathways.is_empty() {
            for pathway in self.pathways.values_mut() {
                pathway.set_datafile(datafile_path);
            }
        }
        // Term Cables
        if !self.term_cables.is_empty() {
            for term_cable in self.term_cables.values_mut() {
                term_cable.set_datafile(datafile_path);
            }
        }
        // Terminal Strips
        if !self.terminal_strips.is_empty() {
            for terminal_strip in self.terminal_strips.values_mut() {
                terminal_strip.set_datafile(datafile_path);
            }
        }
        // Wires
        if !self.wires.is_empty() {
            for wire in self.wires.values_mut() {
                wire.set_datafile(datafile_path);
            }
        }
    }

    /// Validates that all lookup values in project data are present in library or project
    ///
    /// Only run this function after reading in all datafiles into master library.
    ///
    /// # Errors
    ///
    /// Will error if library data or project data referenced in `Project` is not found in the
    /// provided `Library` or referenced `Project`
    #[inline(never)]
    #[expect(clippy::too_many_lines, reason = "its the length it needs to be")]
    pub fn validate(&self, library_data: &Library) -> Result<(), Vec<Error>> {
        let mut errors: Vec<Error> = Vec::new();

        // Cables
        if !self.cables.is_empty() {
            for cable in self.cables.values() {
                if !library_data.cable_types.contains_key(&cable.cable_type) {
                    errors.push(
                        LibraryError::ValueNotFound {
                            id: cable.cable_type.clone(),
                            library_type: "CableType".to_owned(),
                        }
                        .into(),
                    );
                }
            }
        }
        //TODO: finish this
        // Connections
        //if !self.connections.is_empty() {
        //    for connection in self.connections.values() {
        //      check if project key referenced in connection::Type enum exists in the specified
        //      table
        //    }
        //}
        // Connectors
        //if !self.connectors.is_empty() {
        //    for connector in self.connectors.values() {
        //if ! library_data.connector_types.contains_key(&connector.connector_type) {
        //   errors.push(LibraryError::ValueNotFound{
        //        id: connector.connector_type.clone(),
        //        library_type: "ConnectorType".to_owned(),
        //   }.into());
        //}
        //    }
        //}
        // Enclosures
        if !self.enclosures.is_empty() {
            for enclosure in self.enclosures.values() {
                if !library_data.enclosure_types.contains_key(&enclosure.enclosure_type) {
                    errors.push(
                        LibraryError::ValueNotFound {
                            id: enclosure.enclosure_type.clone(),
                            library_type: "EnclosureType".to_owned(),
                        }
                        .into(),
                    );
                }
            }
        }
        // Equipment
        if !self.equipment.is_empty() {
            for equipment in self.equipment.values() {
                if !library_data.equipment_types.contains_key(&equipment.equipment_type) {
                    errors.push(
                        LibraryError::ValueNotFound {
                            id: equipment.equipment_type.clone(),
                            library_type: "EquipmentType".to_owned(),
                        }
                        .into(),
                    );
                }
            }
        }
        // Mounting Rails
        if !self.mounting_rails.is_empty() {
            for mounting_rail in self.mounting_rails.values() {
                if !library_data
                    .mounting_rail_types
                    .contains_key(&mounting_rail.mounting_rail_type)
                {
                    errors.push(
                        LibraryError::ValueNotFound {
                            id: mounting_rail.mounting_rail_type.clone(),
                            library_type: "MountingRailType".to_owned(),
                        }
                        .into(),
                    );
                }
            }
        }
        // Pathways
        if !self.pathways.is_empty() {
            for pathway in self.pathways.values() {
                if !library_data.pathway_types.contains_key(&pathway.path_type) {
                    errors.push(
                        LibraryError::ValueNotFound {
                            id: pathway.path_type.clone(),
                            library_type: "PathwayType".to_owned(),
                        }
                        .into(),
                    );
                }
            }
        }
        // Term Cables
        if !self.term_cables.is_empty() {
            for term_cable in self.term_cables.values() {
                if !library_data.term_cable_types.contains_key(&term_cable.term_cable_type) {
                    errors.push(
                        LibraryError::ValueNotFound {
                            id: term_cable.term_cable_type.clone(),
                            library_type: "TermCableType".to_owned(),
                        }
                        .into(),
                    );
                }
            }
        }
        // Terminal Strips
        if !self.terminal_strips.is_empty() {
            for terminal_strip in self.terminal_strips.values() {
                //Only check this if it is actually defined in the terminal strip
                if let Some(mounting_rail) = &terminal_strip.mounting_rail
                    && !self.mounting_rails.contains_key(mounting_rail)
                {
                    errors.push(
                        ProjectError::ValueNotFound {
                            id: mounting_rail.clone(),
                            project_type: "MountingRail".to_owned(),
                        }
                        .into(),
                    );
                }
                if let Some(enclosure) = &terminal_strip.enclosure
                    && !self.enclosures.contains_key(enclosure)
                {
                    errors.push(
                        ProjectError::ValueNotFound {
                            id: enclosure.clone(),
                            project_type: "Enclosure".to_owned(),
                        }
                        .into(),
                    );
                }
                for element in &terminal_strip.elements {
                    //TODO: also check element.accessories once that is better defined
                    //
                    //TODO: check and see if terminal can accept accessories
                    match &element.contained_type {
                        TermAccy::Terminal(terminal_type) => {
                            if !library_data.terminal_types.contains_key(terminal_type) {
                                errors.push(
                                    LibraryError::ValueNotFound {
                                        id: terminal_type.clone(),
                                        library_type: "TerminalType".to_owned(),
                                    }
                                    .into(),
                                );
                            }
                        }
                        TermAccy::Accessory(accessory_type) => {
                            if !library_data.terminal_strip_accessory_types.contains_key(accessory_type) {
                                errors.push(
                                    LibraryError::ValueNotFound {
                                        id: accessory_type.clone(),
                                        library_type: "TerminalStripAccessoryType".to_owned(),
                                    }
                                    .into(),
                                );
                            }
                        }
                    }
                }
                for jumper in &terminal_strip.jumpers {
                    if !library_data.terminal_strip_jumper_types.contains_key(&jumper.jumper_type) {
                        errors.push(
                            LibraryError::ValueNotFound {
                                id: jumper.jumper_type.clone(),
                                library_type: "TerminalStripJumperType".to_owned(),
                            }
                            .into(),
                        );
                    }
                }
            }
        }
        // Wires
        if !self.wires.is_empty() {
            for wire in self.wires.values() {
                if !library_data.wire_types.contains_key(&wire.wire_type) {
                    errors.push(
                        LibraryError::ValueNotFound {
                            id: wire.wire_type.clone(),
                            library_type: "WireType".to_owned(),
                        }
                        .into(),
                    );
                }
            }
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}

/// Config contains project specific configuration information
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Config {
    /// Name of project
    pub project_name: String,
    /// If default libraries included with the application should be loaded into project
    pub load_default_libraries: bool,
    /// Paths to load libraries from.
    ///
    /// If a path listed is a directory, all `.toml` files within it or subdirectories will be
    /// included as library files. Paths can be either relative or absolute
    ///
    /// Hidden files/directories are ignored.
    pub library_paths: Option<Vec<PathBuf>>,
    /// Specific paths to load project source files from
    ///
    /// If this is not defined, all TOML files in the directory that the cfg file is in, and
    /// sub-directories will be parsed as project files.
    ///
    ///
    /// If a path listed is a directory, all `.toml` files within it or sub-directories will be
    /// treated as project files. Paths can be either relative or absolute
    ///
    /// If this is not defined, then all other TOML files found within the root directory or
    /// subidirectories of the project will be parsed as project files.
    ///
    /// Hidden files/directories are ignored
    pub source_paths: Option<Vec<PathBuf>>,
    //TODO: switch this to an enum
    /// Code reference used for wire ampacity checks and conduit fill, etc.
    /// These are complicated enough that they are currently defined in code
    /// rather than a configuration file.
    pub electrical_code_standard: Option<String>,
    /// IEC project code
    pub project_code: Option<String>,
    /// Optional description of project
    pub description: Option<String>,
}

#[cfg(test)]
mod tests {

    // TODO:  testing ideas (for both project and library):
    // - test import of datafile containing each individual object
    // - test import of basic datafile, minimal amount of data necessary
    // - test import of full datafile, with multiple defined dictionary entries for each dictionary
    // - test failure of multiple of the top level dicts defined in one file
    // - test to make sure only one of wire, cable, termcable is set in project parsing, both
    // positive and negative
    // - test importing a cable/termcable type with a missing wiretype (also for equipment, etc)
    // - test complicated term_cable
    // - test all project datatypes with both present and absent library values
    // - test all panics
    // - test library types that refer to each other, make sure objects are always parsed in
    // correct order
    // - same with project types, except with both library and project assets

    #[test]
    fn read_datafile_project() {}
}
