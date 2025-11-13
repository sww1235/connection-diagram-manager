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
/// `schematic_symbol` represents an instance of `SchematicSymbolType`
pub mod schematic_symbol;
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
    collections::HashMap,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{error::Error, traits::FromFile, util_functions};

/// `Project` represents all project specific data used in program
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Project {
    /// contains all cables read in from files, and/or added in via program logic
    pub cables: HashMap<String, cable::Cable>,
    /// `connections` contains all connections between different equipment/cables/wires
    pub connections: Vec<connection::Connection>,
    //TODO: are connectors going to be read in separately?
    /// contains all connectors read in from files, and/or added in via program logic
    pub connectors: HashMap<String, connector::Connector>,
    /// contains all enclosures read in from files, and/or added in via program logic
    pub enclosures: HashMap<String, enclosure::Enclosure>,
    /// contains all equipment read in from files, and/or added in via program logic
    pub equipment: HashMap<String, equipment::Equipment>,
    /// contains all mounting rails read in from files, and/or added in via program logic
    pub mounting_rails: HashMap<String, mounting_rail::MountingRail>,
    /// contains all pathways read in from files and/or added in via program logic
    pub pathways: HashMap<String, pathway::Pathway>,
    /// contains all schematic symbols read in from files and/or added in via program logic
    pub schematic_symbols: HashMap<String, schematic_symbol::SchematicSymbol>,
    /// contains all term cables read in from files, and/or added in via program logic
    pub term_cables: HashMap<String, term_cable::TermCable>,
    /// contains all terminal strips read in from files and/or added in via program logic
    pub terminal_strips: HashMap<String, terminal_strip::TerminalStrip>,
    /// `wires` contains all wires read in from files, and/or added in via program logic
    pub wires: HashMap<String, wire::Wire>,
}

impl Project {
    /// Merges two instances of `Project`, validating that there are no key conflicts between the
    /// two instances
    ///
    /// # Errors
    ///
    /// Will error if there are duplicate keys found in `other` map
    pub fn merge(&mut self, test_map: Project, test_file: &str) -> Result<(), Error> {
        util_functions::merge_hashmaps(&mut self.cables, test_map.cables, test_file)?;
        self.connections.extend(test_map.connections);
        //TODO: are connectors going to be read in separately?
        //util_functions::merge_hashmaps(&mut self.connectors, test_map.connectors, test_file)?;
        util_functions::merge_hashmaps(&mut self.enclosures, test_map.enclosures, test_file)?;
        util_functions::merge_hashmaps(&mut self.equipment, test_map.equipment, test_file)?;
        util_functions::merge_hashmaps(&mut self.mounting_rails, test_map.mounting_rails, test_file)?;
        util_functions::merge_hashmaps(&mut self.pathways, test_map.pathways, test_file)?;
        util_functions::merge_hashmaps(&mut self.schematic_symbols, test_map.schematic_symbols, test_file)?;
        util_functions::merge_hashmaps(&mut self.term_cables, test_map.term_cables, test_file)?;
        util_functions::merge_hashmaps(&mut self.terminal_strips, test_map.terminal_strips, test_file)?;
        util_functions::merge_hashmaps(&mut self.wires, test_map.wires, test_file)?;
        Ok(())
    }
    /// Inserts datafile path into all structs in the called project
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
        // Schematic Symbols
        if !self.schematic_symbols.is_empty() {
            for schematic_symbol in self.schematic_symbols.values_mut() {
                schematic_symbol.set_datafile(datafile_path);
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
}

/// Config contains project specific configuration information
#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
