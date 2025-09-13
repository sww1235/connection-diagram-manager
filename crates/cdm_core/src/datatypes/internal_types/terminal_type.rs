use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::datatypes::{
    color::Color,
    internal_types::svg::Svg,
    unit_helper::CrossSectionalArea,
    util_types::{Catalog, Dimension},
};

//TODO:

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
    /// Dimensional information of terminal
    pub dimensions: Option<Dimension>,
    /// Primary color of terminal
    pub color: Option<Color>,
    /// Component Designator
    pub component_designator: Option<String>,
    /// Secondary color of terminal
    /// Mainly used for PE terminal blocks
    pub secondary_color: Option<Color>,
    /// If this terminal type accepts plug in accessories
    /// like fuses or component holders
    pub accepts_accessories: bool,
    /// Indicates if this terminal has an integrated fuse
    pub fuse_terminal: bool,
    /// rating of integrated fuse. Not parsed
    ///
    /// use accessory if fuse is pluggable
    pub fuse_rating: Option<String>,
    /// Indicates if this terminal has an integrated visual indicator
    pub indicator_present: bool,
    /// Indicator rating. Not parsed
    pub indicator_rating: Option<String>,
    /// Indicator type. Not parsed
    ///
    /// LED, incandecent, neon, etc
    pub indicator_type: Option<String>,
    /// Indicates that this terminal has an integrated discrete component.
    ///
    /// This discrete component is either non-removable or not easily replaceable
    /// If the component is replacable or pluggable, use an accessory
    pub discrete_component_present: bool,
    /// Rating of discrete component. Not parsed
    pub discrete_component_rating: Option<String>,
    /// Type of discrete component: Resistor, Diode, etc.
    pub discrete_component_type: Option<String>,
    /// If there is an integrated, non-removable disconnect present
    ///
    /// If the disconnect is removable, use an accessory instead
    pub integrated_disconnect_present: Option<String>,
    /// Visual representation of `TerminalType`
    pub visual_representation: Option<Svg>,
    /// HashMap defining terminal layers
    /// at least 1 layer is required for a terminal
    pub layers: HashMap<String, Layer>,
    /// Which terminal connections are connected
    pub internal_connections: Vec<InternalConnection>,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}

/// `Layer` represents one layer of a terminal
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Layer {
    /// Vector of `Connection`s on a particular layer of a terminal
    pub connections: Vec<Connection>,
}

/// `Connection` represents one wire connection point on a terminal layer.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Connection {
    /// connection designation
    /// must be unique among connection points on a layer
    /// only used as a reference in the `internal_connections` section below
    pub connection_description: String,
    /// Connection Type of connection
    ///
    /// Possible Values:
    /// - ScrewTerminal
    /// - Bolt
    /// - PlugIn
    /// - PushIn
    /// - QuickConnect
    /// - Spade
    /// - SpringCage
    /// - LeverLock
    pub connection_type: ConnectionType,
    pub entry_angle: Option<String>,
    /// maximum number of wires allowed to be connected to this terminal connection
    /// can be lower than manufacturer recommended values
    pub maximum_wires: u64,
    /// Maximum Wire Cross Section that can be connected to this terminal connection
    pub maximum_wire_cross_section: CrossSectionalArea,
    /// Minimum Wire Cross Section that can be connected to this terminal connection
    pub minimum_wire_cross_section: CrossSectionalArea,
    /// Different Wire Type or connector Type accepted
    ///
    /// Note: This is not an actual wire/connector type but
    /// just a comparison string.
    ///
    /// Possible Values:
    /// - Solid
    /// - Stranded
    /// - StrandedFerrule
    /// - Spade
    pub wire_types_accepted: Vec<AcceptedWireType>,
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

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum AcceptedWireType {
    Solid,
    Stranded,
    StrandedFerrule,
    Spade,
}

/// Internal Connections within a terminal
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct InternalConnection {
    /// which connection points on a terminal are connected
    pub connected_connections: Vec<String>,
    /// used to indicate a connection from this set of internal connections
    /// to the mounting rail.
    /// mainly used for PE/grounding terminal blocks.
    pub mount_connection: bool,
}

/// `TerminalStripJumperType` represents a manufactured jumper
/// that jumps between multiple terminals in a `TerminalStrip`
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TerminalStripJumperType {
    /// Catalog information
    pub catalog: Option<Catalog>,
    /// Dimensional information of jumper
    pub dimensions: Option<Dimension>,
    /// Vector of `TerminalType` IDs
    pub compatible_terminal_type: Vec<String>,
    /// Number of terminal positions
    pub number_of_positions: u64,
    /// color of jumper
    pub color: Option<Color>,
    /// Visual representation of `TerminalStripJumperType`
    pub visual_representation: Option<Svg>,
    /// per pin compatible terminal_block_types
    /// specify an array of terminal_block_types per pin
    pub pin_compatible_terminal_types: Option<Vec<Vec<String>>>,
}

/// `TerminalAccessoryType` represents Terminal accessories are items that insert into a terminal
/// like fuse holders, component holders, disconnect switches, etc
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TerminalAccessoryType {
    /// Catalog information
    pub catalog: Option<Catalog>,
    /// Dimensional information of accessory
    pub dimensions: Option<Dimension>,
    /// Compatible terminal type ids
    pub compatible_terminal_type: Vec<String>,
    /// Accessory supertype:
    ///
    /// Fuse, component carrier, disconect blade, etc
    pub accessory_supertype: String,
    /// Visual representation of `TerminalAccessoryType`
    pub visual_representation: Option<Svg>,
    /// color of accessory
    pub color: Option<Color>,
}

/// `TerminalStripAccessoryType` represents Terminal strip accessories
///
/// Terminal strip accessories are things like end plates or spacers that are incorporated into a
/// terminal_strip linearly and interface with terminals This does not include things like DIN rail
/// stops.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TerminalStripAccessoryType {
    /// Catalog information
    pub catalog: Option<Catalog>,
    /// Dimensional information of accessory
    pub dimensions: Option<Dimension>,
    /// Compatible terminal type ids
    pub compatible_terminal_type: Vec<String>,
    /// Accessory supertype:
    ///
    /// Fuse, component carrier, disconect blade, etc
    pub accessory_supertype: String,
    /// Visual representation of `TerminalAccessoryType`
    pub visual_representation: Option<Svg>,
    /// color of accessory
    pub color: Option<Color>,
}
