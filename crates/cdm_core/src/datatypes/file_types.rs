// reference for this idea
// https://users.rust-lang.org/t/how-to-merge-slotmaps/139907/6

#![expect(missing_docs, reason = "self documenting")]

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

//TODO: validate SVG during TryFrom impls
pub mod cable;
pub mod connection;
pub mod connector;
pub mod enclosure;
pub mod equipment;
pub mod mounting_rail;
pub mod pathway;
pub mod term_cable;
pub mod terminal_strip;

pub mod cable_type;
pub mod connector_type;
pub mod enclosure_type;
pub mod equipment_type;
pub mod mounting_rail_type;
pub mod pathway_type;
pub mod schematic_symbol_type;
pub mod term_cable_type;
pub mod terminal_type;

/// `Project` is the source code representation of the on-disk file format for an in-memory
/// [`Project`](crate::datatypes::project_types::Project).
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(default)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct Project {
    /// contains all cables read in from files, and/or added in via program logic.
    pub cables: BTreeMap<String, cable::Cable>,
    /// `connections` contains all connections between different equipment/cables/wires.
    pub connections: Vec<connection::Connection>,
    //TODO: are connectors going to be read in separately?
    /// contains all connectors read in from files, and/or added in via program logic.
    pub connectors: BTreeMap<String, connector::Connector>,
    /// contains all enclosures read in from files, and/or added in via program logic.
    pub enclosures: BTreeMap<String, enclosure::Enclosure>,
    /// contains all equipment read in from files, and/or added in via program logic.
    pub equipment: BTreeMap<String, equipment::Equipment>,
    /// contains all mounting rails read in from files, and/or added in via program logic.
    pub mounting_rails: BTreeMap<String, mounting_rail::MountingRail>,
    /// contains all pathways read in from files and/or added in via program logic.
    pub pathways: BTreeMap<String, pathway::Pathway>,
    /// contains all term cables read in from files, and/or added in via program logic.
    pub term_cables: BTreeMap<String, term_cable::TermCable>,
    /// contains all terminal strips read in from files and/or added in via program logic.
    pub terminal_strips: BTreeMap<String, terminal_strip::TerminalStrip>,
}

/// `Library` is the source code representation of the on-disk file format for an in-memory
/// [`Library`](crate::datatypes::library_types::Library).
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct Library {
    /// contains all cable types read in from file, and/or added in via program logic.
    pub cable_types: BTreeMap<String, cable_type::CableType>,
    /// contains all connector types read in from file, and/or added in via program logic.
    pub connector_types: BTreeMap<String, connector_type::ConnectorType>,
    /// contains all enclosure types read in from file, and/or added in via program logic.
    pub enclosure_types: BTreeMap<String, enclosure_type::EnclosureType>,
    /// contains all equipment types read in from file, and/or added in via program logic.
    pub equipment_types: BTreeMap<String, equipment_type::EquipmentType>,
    /// contains all mounting rail types read in from file, and/or added in via program logic.
    pub mounting_rail_types: BTreeMap<String, mounting_rail_type::MountingRailType>,
    /// contains all pathway types read in from file, and/or added in via program logic.
    pub pathway_types: BTreeMap<String, pathway_type::PathwayType>,
    /// contains all schematic symbol types read in from file and/or added in via program logic.
    pub schematic_symbol_types: BTreeMap<String, schematic_symbol_type::SchematicSymbolType>,
    /// contains all terminated cable types read in from file, and/or added in via program logic.
    pub term_cable_types: BTreeMap<String, term_cable_type::TermCableType>,
    /// contains all terminal types read in from file, and/or added in via program logic.
    pub terminal_types: BTreeMap<String, terminal_type::TerminalType>,
    /// contains all terminal strip jumper types read in from file, and/or added in via program
    /// logic.
    pub terminal_strip_jumper_types: BTreeMap<String, terminal_type::TerminalStripJumperType>,
    /// contains all terminal accessory types read in from file, and/or added in via program logic.
    pub terminal_accessory_types: BTreeMap<String, terminal_type::TerminalAccessoryType>,
    /// contains all terminal strip accessory types read in from file, and/or added in via program
    /// logic.
    pub terminal_strip_accessory_types: BTreeMap<String, terminal_type::TerminalStripAccessoryType>,
}
