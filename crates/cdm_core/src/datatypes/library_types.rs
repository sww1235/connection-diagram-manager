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

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{error::Error, traits::FromFile, util_functions};

/// `Library` represents all library data used in program
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
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
    /// contains all schematic symbol types read in from file and/or added in via program logic
    pub schematic_symbol_types: HashMap<String, schematic_symbol_type::SchematicSymbolType>,
    /// contains all terminated cable types read in from file, and/or added in via program logic
    pub term_cable_types: HashMap<String, term_cable_type::TermCableType>,
    /// contains all terminal types read in from file, and/or added in via program logic
    pub terminal_types: HashMap<String, terminal_type::TerminalType>,
    /// contains all terminal strip jumper types read in from file, and/or added in via program
    /// logic
    pub terminal_strip_jumper_types: HashMap<String, terminal_type::TerminalStripJumperType>,
    /// contains all terminal accessory types read in from file, and/or added in via program logic
    pub terminal_accessory_types: HashMap<String, terminal_type::TerminalAccessoryType>,
    /// contains all terminal strip accessory types read in from file, and/or added in via program
    /// logic
    pub terminal_strip_accessory_types: HashMap<String, terminal_type::TerminalStripAccessoryType>,
    /// contains all wire types read in from file, and/or added in via program logic
    pub wire_types: HashMap<String, wire_type::WireType>,
}

impl Library {
    /// Merges two instances of `Library`, validating that there are no key conflicts between the
    /// two instances
    ///
    /// # Errors
    ///
    /// Will error if there are duplicate keys found in `other` map
    pub fn merge(&mut self, test_map: Library, test_file: &str) -> Result<(), Error> {
        util_functions::merge_hashmaps(&mut self.cable_types, test_map.cable_types, test_file)?;
        util_functions::merge_hashmaps(&mut self.connector_types, test_map.connector_types, test_file)?;
        util_functions::merge_hashmaps(&mut self.enclosure_types, test_map.enclosure_types, test_file)?;
        util_functions::merge_hashmaps(&mut self.equipment_types, test_map.equipment_types, test_file)?;
        util_functions::merge_hashmaps(&mut self.mounting_rail_types, test_map.mounting_rail_types, test_file)?;
        util_functions::merge_hashmaps(&mut self.pathway_types, test_map.pathway_types, test_file)?;
        util_functions::merge_hashmaps(&mut self.term_cable_types, test_map.term_cable_types, test_file)?;
        util_functions::merge_hashmaps(&mut self.terminal_types, test_map.terminal_types, test_file)?;
        util_functions::merge_hashmaps(
            &mut self.terminal_strip_jumper_types,
            test_map.terminal_strip_jumper_types,
            test_file,
        )?;
        util_functions::merge_hashmaps(
            &mut self.terminal_accessory_types,
            test_map.terminal_accessory_types,
            test_file,
        )?;
        util_functions::merge_hashmaps(
            &mut self.terminal_strip_accessory_types,
            test_map.terminal_strip_accessory_types,
            test_file,
        )?;
        util_functions::merge_hashmaps(&mut self.wire_types, test_map.wire_types, test_file)?;
        Ok(())
    }

    /// Inserts datafile path into all structs in the called library
    pub fn add_datafile_paths(&mut self, datafile_path: &Path) {
        // Cable Types
        if !self.cable_types.is_empty() {
            for cable_type in self.cable_types.values_mut() {
                cable_type.set_datafile(datafile_path);
            }
        }
        // Connector Types
        if !self.connector_types.is_empty() {
            for connector_type in self.connector_types.values_mut() {
                connector_type.set_datafile(datafile_path);
            }
        }
        // Enclosure Types
        if !self.enclosure_types.is_empty() {
            for enclosure_type in self.enclosure_types.values_mut() {
                enclosure_type.set_datafile(datafile_path);
            }
        }
        // Equipment Types
        if !self.equipment_types.is_empty() {
            for equipment_type in self.equipment_types.values_mut() {
                equipment_type.set_datafile(datafile_path);
            }
        }
        // Mounting Rail Types
        if !self.mounting_rail_types.is_empty() {
            for mounting_rail_type in self.mounting_rail_types.values_mut() {
                mounting_rail_type.set_datafile(datafile_path);
            }
        }
        // Pathway Types
        if !self.pathway_types.is_empty() {
            for pathway_type in self.pathway_types.values_mut() {
                pathway_type.set_datafile(datafile_path);
            }
        }
        // Schematic Symbol Types
        if !self.schematic_symbol_types.is_empty() {
            for schematic_symbol_type in self.schematic_symbol_types.values_mut() {
                schematic_symbol_type.set_datafile(datafile_path);
            }
        }
        // Term Cable Types
        if !self.term_cable_types.is_empty() {
            for term_cable_type in self.term_cable_types.values_mut() {
                term_cable_type.set_datafile(datafile_path);
            }
        }
        // Terminal Types
        if !self.terminal_types.is_empty() {
            for terminal_type in self.terminal_types.values_mut() {
                terminal_type.set_datafile(datafile_path);
            }
        }
        // Terminal Strip Jumper Types
        if !self.terminal_strip_jumper_types.is_empty() {
            for terminal_strip_jumper_type in self.terminal_strip_jumper_types.values_mut() {
                terminal_strip_jumper_type.set_datafile(datafile_path);
            }
        }
        // Terminal Accessory Types
        if !self.terminal_accessory_types.is_empty() {
            for terminal_accessory_type in self.terminal_accessory_types.values_mut() {
                terminal_accessory_type.set_datafile(datafile_path);
            }
        }
        // Terminal Strip Accessory Types
        if !self.terminal_strip_accessory_types.is_empty() {
            for terminal_strip_accessory_type in self.terminal_strip_accessory_types.values_mut() {
                terminal_strip_accessory_type.set_datafile(datafile_path);
            }
        }
        // Wire Types
        if !self.wire_types.is_empty() {
            for wire_type in self.wire_types.values_mut() {
                wire_type.set_datafile(datafile_path);
            }
        }
    }
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
    fn read_datafile_library() {}
}
