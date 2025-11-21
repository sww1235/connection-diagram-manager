use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{
    datatypes::{
        color::Color,
        svg::Svg,
        unit_helper::CrossSectionalArea,
        util_types::{Catalog, Dimension},
    },
    traits::FromFile,
};

//TODO:

/// `TerminalType` represents a terminal for connecting wires together.
/// Terminals are separated out into their own category due to some special case things with them,
/// including the accessories, and ganging.
/// Terminal definitions include both DIN rail mounted terminals, WAGO lever nuts, and Wire nuts
/// Ferrules, ring/space/fork terminals, etc should be defined as connectors since they associate
/// with wires
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[expect(clippy::struct_excessive_bools, clippy::partial_pub_fields)]
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
    /// `BTreeMap` defining terminal layers
    /// at least 1 layer is required for a terminal
    pub layers: BTreeMap<String, Layer>,
    /// Which terminal connections are connected
    pub internal_connections: Vec<InternalConnection>,
    /// datafile the struct instance was read in from
    #[serde(skip)]
    pub(crate) contained_datafile_path: PathBuf,
}
impl FromFile for TerminalType {
    fn datafile(&self) -> PathBuf {
        self.contained_datafile_path.clone()
    }
    fn set_datafile(&mut self, datafile_path: &Path) {
        self.contained_datafile_path = datafile_path.to_path_buf();
    }
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
    /// - `ScrewTerminal`
    /// - `Bolt`
    /// - `PlugIn`
    /// - `PushIn`
    /// - `PushInX`
    /// - `FastConnect`
    /// - `Spade`
    /// - `SpringCage`
    /// - `LeverLock`
    pub connection_type: ConnectionType,
    /// Connector entry angle
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
    /// - `Solid`
    /// - `Stranded`
    /// - `FineStranded`
    /// - `StrandedFerrule`
    /// - `Spade`
    pub wire_types_accepted: Vec<AcceptedWireType>,
}

/// `ConnectionType` represents different types of connections that can be on a terminal
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ConnectionType {
    /// `ScrewTerminal` uses a threaded screw to clamp the conductor
    ScrewTerminal,
    /// `Bolt` connections use a bolt to attach a lug to the terminal
    Bolt,
    /// `PlugIn` connections use a separable connector to connect wires to the terminal. The
    /// separable connector will have another type of `ConnectionType`
    PlugIn,
    /// `PushIn` connections use a constant force spring to clamp the conductor. They do not require
    /// the use of tools to insert the wire. They will have a button that can be pressed with a
    /// small screw driver to remove the terminal.
    ///
    /// The wire entry is on the top of the terminal
    PushIn,
    /// `PushInX` connections are identical in function to `PushIn` connections but the wire entry
    /// is on the side of the terminal
    PushInX,
    /// `FastConnect` connections use a lever mechanism to clamp the connector
    FastConnect,
    /// `Spade` connections accept spade terminals crimped onto the wires
    Spade,
    /// `SpringCage` connections use a spring to maintain the connection to the wire. They require
    /// a screwdriver to insert and remove the wire
    SpringCage,
    /// `LeverLock` connections use a over center lever mechanism to clamp the wires.
    LeverLock,
}

/// `AcceptedWireType` shows which types of wires terminals can accept
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AcceptedWireType {
    /// Solid core wire. One strand
    Solid,
    /// Stranded wire
    Stranded,
    /// Finely stranded wire
    FineStranded,
    /// Stranded wire with a crimped ferrule
    StrandedFerrule,
    /// wire with a crimped spade terminal
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
#[expect(clippy::partial_pub_fields)]
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
    /// per pin compatible `terminal_block_type`s
    /// specify an array of `terminal_block_type`s per pin
    pub pin_compatible_terminal_types: Option<Vec<Vec<String>>>,
    /// datafile the struct instance was read in from
    #[serde(skip)]
    pub(super) contained_datafile_path: PathBuf,
}
impl FromFile for TerminalStripJumperType {
    fn datafile(&self) -> PathBuf {
        self.contained_datafile_path.clone()
    }
    fn set_datafile(&mut self, datafile_path: &Path) {
        self.contained_datafile_path = datafile_path.to_path_buf();
    }
}

/// `TerminalAccessoryType` represents Terminal accessories are items that insert into a terminal
/// like fuse holders, component holders, disconnect switches, etc
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[expect(clippy::partial_pub_fields)]
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
    /// datafile the struct instance was read in from
    #[serde(skip)]
    pub(super) contained_datafile_path: PathBuf,
}
impl FromFile for TerminalAccessoryType {
    fn datafile(&self) -> PathBuf {
        self.contained_datafile_path.clone()
    }
    fn set_datafile(&mut self, datafile_path: &Path) {
        self.contained_datafile_path = datafile_path.to_path_buf();
    }
}

/// `TerminalStripAccessoryType` represents Terminal strip accessories
///
/// Terminal strip accessories are things like end plates or spacers that are incorporated into a
/// `terminal_strip` linearly and interface with terminals This does not include things like DIN
/// rail stops.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[expect(clippy::partial_pub_fields)]
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
    /// datafile the struct instance was read in from
    #[serde(skip)]
    pub(super) contained_datafile_path: PathBuf,
}
impl FromFile for TerminalStripAccessoryType {
    fn datafile(&self) -> PathBuf {
        self.contained_datafile_path.clone()
    }
    fn set_datafile(&mut self, datafile_path: &Path) {
        self.contained_datafile_path = datafile_path.to_path_buf();
    }
}
