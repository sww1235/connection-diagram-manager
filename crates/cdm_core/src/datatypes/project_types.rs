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

use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{error::Error, util_functions};

/// `Project` represents all project specific data used in program
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Project {
    /// contains all cables read in from files, and/or added in via program logic
    pub cables: HashMap<String, cable::Cable>,
    /// `connections` contains all connections between different equipment/cables/wires
    pub connections: Vec<connection::Connection>,
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
    pub fn merge(&mut self, other: Project, file1: &str, file2: &str) -> Result<(), Error> {
        util_functions::merge_hashmaps(&mut self.cables, other.cables, file1, file2)?;
        self.connections.extend(other.connections);
        util_functions::merge_hashmaps(&mut self.connectors, other.connectors, file1, file2)?;
        util_functions::merge_hashmaps(&mut self.enclosures, other.enclosures, file1, file2)?;
        util_functions::merge_hashmaps(&mut self.equipment, other.equipment, file1, file2)?;
        util_functions::merge_hashmaps(&mut self.mounting_rails, other.mounting_rails, file1, file2)?;
        util_functions::merge_hashmaps(&mut self.pathways, other.pathways, file1, file2)?;
        util_functions::merge_hashmaps(&mut self.schematic_symbols, other.schematic_symbols, file1, file2)?;
        util_functions::merge_hashmaps(&mut self.term_cables, other.term_cables, file1, file2)?;
        util_functions::merge_hashmaps(&mut self.terminal_strips, other.terminal_strips, file1, file2)?;
        util_functions::merge_hashmaps(&mut self.wires, other.wires, file1, file2)?;
        Ok(())
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
