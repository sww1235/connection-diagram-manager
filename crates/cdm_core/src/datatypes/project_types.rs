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

/// `Project` represents all project specific data used in program
#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
