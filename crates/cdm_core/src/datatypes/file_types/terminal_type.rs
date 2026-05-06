use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::datatypes::{
    color::Color,
    library_types::terminal_type::{InternalConnection, Layer},
    svg::Svg,
    util_types::{Catalog, Dimension},
};

/// `TerminalType` is the source code representation of the on-disk file format for an in-memory
/// [`TerminalType`](crate::datatypes::library_types::terminal_type::TerminalType).
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[expect(clippy::struct_excessive_bools, reason = "lots of boolean values")]
#[non_exhaustive]
pub struct TerminalType {
    /// Catalog information.
    pub catalog: Option<Catalog>,
    /// Dimensional information of terminal.
    pub dimensions: Option<Dimension>,
    /// Primary color of terminal.
    pub color: Option<Color>,
    /// Component Designator.
    pub component_designator: Option<String>,
    /// Secondary color of terminal.
    ///
    /// Mainly used for PE terminal blocks.
    pub secondary_color: Option<Color>,
    /// If this terminal type accepts plug in accessories
    /// like fuses or component holders.
    pub accepts_accessories: bool,
    /// Indicates if this terminal has an integrated fuse.
    pub fuse_terminal: bool,
    /// Rating of integrated fuse. Not parsed.
    ///
    /// Use accessory if fuse is pluggable.
    pub fuse_rating: Option<String>,
    /// Indicates if this terminal has an integrated visual indicator.
    pub indicator_present: bool,
    /// Indicator rating. Not parsed.
    pub indicator_rating: Option<String>,
    /// Indicator type. Not parsed.
    ///
    /// LED, incandecent, neon, etc.
    pub indicator_type: Option<String>,
    /// Indicates that this terminal has an integrated discrete component.
    ///
    /// This discrete component is either non-removable or not easily replaceable
    /// If the component is replacable or pluggable, use an accessory.
    pub discrete_component_present: bool,
    /// Rating of discrete component. Not parsed.
    pub discrete_component_rating: Option<String>,
    /// Type of discrete component: Resistor, Diode, etc.
    pub discrete_component_type: Option<String>,
    /// If there is an integrated, non-removable disconnect present.
    ///
    /// If the disconnect is removable, use an accessory instead.
    pub integrated_disconnect_present: bool,
    /// Visual representation of `TerminalType`.
    pub visual_representation: Option<Svg>,
    /// Vector of schematic symbols that can represent this terminal.
    /// values must be the id of the `symbol_type`.
    #[serde(default)]
    pub schematic_symbols: Vec<String>,
    /// `BTreeMap` defining terminal layers.
    /// At least 1 layer is required for a terminal.
    pub layers: BTreeMap<String, Layer>,
    /// Which terminal connections are connected.
    pub internal_connections: Vec<InternalConnection>,
}

/// `TerminalStripJumperType` is the source code representation of the on-disk file format for an in-memory
/// [`TerminalStripJumperType`](crate::datatypes::library_types::terminal_type::TerminalStripJumperType).
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TerminalStripJumperType {
    /// Catalog information.
    pub catalog: Option<Catalog>,
    /// Dimensional information of jumper.
    pub dimensions: Option<Dimension>,
    /// Vector of `TerminalType` IDs.
    pub compatible_terminal_types: Vec<String>,
    /// Number of terminal positions.
    pub number_of_positions: u64,
    /// color of jumper.
    pub color: Option<Color>,
    /// Visual representation of `TerminalStripJumperType`.
    pub visual_representation: Option<Svg>,
    /// Vector of schematic symbols that can represent this terminal strip jumper type.
    /// values must be the id of the `symbol_type`.
    #[serde(default)]
    pub schematic_symbols: Vec<String>,
    /// Per pin compatible `TerminalType`s.
    ///
    /// Specify an array of `TerminalType`s per pin.
    /// The outer array is pin numbers.
    #[serde(default)]
    pub pin_compatible_terminal_types: Vec<Vec<String>>,
}

/// `TerminalAccessoryType` is the source code representation of the on-disk file format for an in-memory
/// [`TerminalAccessoryType`](crate::datatypes::library_types::terminal_type::TerminalAccessoryType).
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TerminalAccessoryType {
    /// Catalog information.
    pub catalog: Option<Catalog>,
    /// Dimensional information of accessory.
    pub dimensions: Option<Dimension>,
    /// Compatible terminal type ids.
    pub compatible_terminal_types: Vec<String>,
    /// Accessory supertype:
    ///
    /// Fuse, component carrier, disconect blade, etc.
    pub accessory_supertype: String,
    /// Visual representation of `TerminalAccessoryType`.
    pub visual_representation: Option<Svg>,
    /// Vector of schematic symbols that can represent this terminal accessory type.
    /// values must be the id of the `SchematicSymbolType`.
    #[serde(default)]
    pub schematic_symbols: Vec<String>,
    /// Color of accessory.
    pub color: Option<Color>,
}

/// `TerminalStripAccessoryType` is the source code representation of the on-disk file format for an in-memory
/// [`TerminalStripAccessoryType`](crate::datatypes::library_types::terminal_type::TerminalStripAccessoryType).
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TerminalStripAccessoryType {
    /// Catalog information.
    pub catalog: Option<Catalog>,
    /// Dimensional information of accessory.
    pub dimensions: Option<Dimension>,
    /// Compatible terminal type ids.
    pub compatible_terminal_types: Vec<String>,
    /// Accessory supertype:
    ///
    /// Fuse, component carrier, disconect blade, etc.
    pub accessory_supertype: String,
    /// Visual representation of `TerminalAccessoryType`.
    pub visual_representation: Option<Svg>,
    /// Vector of schematic symbols that can represent this terminal strip accessory type.
    /// values must be the id of the `SchematicSymbolType`.
    #[serde(default)]
    pub schematic_symbols: Vec<String>,
    /// Color of accessory.
    pub color: Option<Color>,
}
