///`cable_type` represents a cable with multiple cores
pub mod cable_type;
/// `connector_type` represents a connector
pub mod connector_type;
/// `enclosure_type` represents a type of enclosure
pub mod enclosure_type;
/// `equipment_type` represents a type of equipment
pub mod equipment_type;
/// `mounting_rail_type` represents a type of mounting rail such as DIN rail or unistrut
pub mod mounting_rail_type;
/// `pathway_type` represents a type of pathway for wires or cables
pub mod pathway_type;
/// `schematic_symbol_type` represents a type of schematic symbol
pub mod schematic_symbol_type;
/// `term_cable_type` represents a cable that has connectors assembled on to it
pub mod term_cable_type;
/// `terminal_type` represents a type of terminal
/// This module also includes related defintions including accessories and jumpers
pub mod terminal_type;
/// `wire_type` represents an individual wire with optional insulation
pub mod wire_type;

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
/// `physical_location` represents a physical location in the world
pub mod physical_location;
/// `schematic_symbol` represents an instance of `SchematicSymbolType`
pub mod schematic_symbol;
/// `term_cable` represents an instance of a `TermCableType`
pub mod term_cable;
/// `terminal_strip` contains the main terminal_strip type and accessory types
pub mod terminal_strip;
/// `wire` represents an instance of a `WireType`
pub mod wire;

// TODO: improve this documentation
/// `connection` represents a connection between two different elements
pub mod connection;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

//TODO: fix all display methods to use proper units
//
//TODO: add additional library/project types into their associated library structs

/// `Library` represents all library data used in program
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Library {
    /// contains all cable types read in from file, and/or added in via program logic
    pub cable_types: HashMap<String, cable_type::CableType>,
    /// contains all connector types read in from file, and/or added in via program logic
    pub connector_types: HashMap<String, connector_type::ConnectorType>,
    /// contains all enclosure types read in from file, and/or added in via program logic
    pub enclosure_types: HashMap<String, enclosure_type::EnclosureType>,
    /// contains all equipment types read in from file, and/or added in via program logic
    pub equipment_types: HashMap<String, equipment_type::EquipmentType>,
    /// contains all mounting rail types read in from file, and/or added in via program logic
    pub mounting_rail_types: HashMap<String, mounting_rail_type::MountingRailType>,
    /// contains all pathway types read in from file, and/or added in via program logic
    pub pathway_types: HashMap<String, pathway_type::PathwayType>,
    /// contains all terminated cable types read in from file, and/or added in via program logic
    pub term_cable_types: HashMap<String, term_cable_type::TermCableType>,
    /// contains all terminal types read in from file, and/or added in via program logic
    pub terminal_types: HashMap<String, terminal_type::TerminalType>,
    /// contains all terminal strip jumper types read in from file, and/or added in via program logic
    pub terminal_strip_jumper_types: HashMap<String, terminal_type::TerminalStripJumperType>,
    /// contains all terminal accessory types read in from file, and/or added in via program logic
    pub terminal_accessory_types: HashMap<String, terminal_type::TerminalAccessoryType>,
    /// contains all terminal strip accessory types read in from file, and/or added in via program logic
    pub terminal_strip_accessory_types: HashMap<String, terminal_type::TerminalStripAccessoryType>,
    /// contains all wire types read in from file, and/or added in via program logic
    pub wire_types: HashMap<String, wire_type::WireType>,
}

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

impl Library {
    ///Initializes an empty `Library`
    #[must_use]
    pub fn new() -> Self {
        Library {
            wire_types: HashMap::new(),
            cable_types: HashMap::new(),
            term_cable_types: HashMap::new(),
            location_types: HashMap::new(),
            connector_types: HashMap::new(),
            equipment_types: HashMap::new(),
            pathway_types: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;

    #[test]
    fn new_library() {
        assert_eq!(
            Library::new(),
            Library {
                wire_types: HashMap::new(),
                cable_types: HashMap::new(),
                term_cable_types: HashMap::new(),
                location_types: HashMap::new(),
                connector_types: HashMap::new(),
                equipment_types: HashMap::new(),
                pathway_types: HashMap::new(),
            }
        )
    }

    #[test]
    fn new_project() {
        assert_eq!(
            Project::new(),
            Project {
                locations: HashMap::new(),
                equipment: HashMap::new(),
                pathways: HashMap::new(),
                wire_cables: HashMap::new(),
            }
        )
    }

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
    fn read_datafile_library() {}

    #[test]
    fn read_datafile_project() {}
}
