use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uom::si::rational64::Area;

use crate::datatypes::{
    color::Color,
    internal_types::svg::Svg,
    util_types::{Catalog, Dimension},
};

/// `TerminalType` represents a terminal for connecting wires together.
/// Terminals are separated out into their own category due to some special case things with them,
/// including the accessories, and ganging.
/// Terminal definitions include both DIN rail mounted terminals, WAGO lever nuts, and Wire nuts
/// Ferrules, ring/space/fork terminals, etc should be defined as connectors since they associate
/// with wires
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TerminalType {
    /// Catalog information
    pub catalog: Option<Catalog>,
    /// Dimensional information of equipment
    pub dimensions: Option<Dimension>,
    /// Primary color of terminal
    pub color: Option<Color>,
    /// Component Designator
    pub component_designator: Option<String>,
    /// Secondary color of terminal
    /// Mainly used for PE terminal blocks
    pub secondary_color: Option<Color>,
    /// Visual representation of Enclosure
    pub visual_representation: Option<Svg>,
    /// HashMap defining terminal layers
    /// at least 1 layer is required for a terminal
    pub layers: HashMap<String, Layer>,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}

/// `Layer` represents one layer of a terminal
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub connections: Vec<Connection>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Connection {
    /// connection designation
    /// must be unique among connection points on a layer
    /// only used as a reference in the `internal_connections` section below
    pub connection_description: String,
    pub connection_type: ConnectionType,
    pub entry_angle: Option<String>,
    /// maximum number of wires allowed to be connected to this terminal.
    /// can be lower than manufacturer recommended values
    pub maximum_wires: u64,
    pub maximum_wire_cross_section: Area,
    maximum_wire_cross_section_unit: String,
    pub minimum_wire_cross_section: Area,
    minimum_wire_cross_section_unit: String,
    pub wire_types_accepted: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ConnectionType {
    ScrewTerminal,
    Bolt,
    PlugIn,
    PushIn,
    QuickConnect,
    Spade,
    SpringCage,
    LeverLock,
}
