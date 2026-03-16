use std::collections::BTreeMap;

use bitflags::bitflags;

use super::svg::Svg;

/// `SchematicSymbol` represents an instance of a `SchematicSymbolType`.
///
/// It contains data and information about a specific symbol in the schematic.
///
/// It is used to hold data for specific instances of symbols that may be updated during the use of
/// the application.
#[derive(Debug, PartialEq, Clone, Default)]
#[non_exhaustive]
pub struct SchematicSymbol {
    /// ID of `SymbolType` in library.
    pub symbol_type: String,
    /// Raw SVG data.
    pub visual_representation: Svg,
    // TODO: keep?
    /// Identifier of symbol.
    pub identifier: String,
    /// Map of connection points on symbol. The ID must be unique per symbol.
    pub connections: BTreeMap<String, SymbolConnection>,
}

//impl Widget for &mut SchematicSymbol {
//    fn ui(self, ui: &mut Ui) -> Response {
//        let sense_settings = Sense::DRAG & Sense::FOCUSABLE;
//        let image =
// Image::from_bytes(self.uri(),self.visual_representation.get_data().into_bytes()).sense(sense_settings).
//
//    }
//
//}
//
//impl SchematicSymbol {
//    pub fn uri(&self) -> String {
//
//        format!("bytes://{}_schematic_symbol.svg", self.identifier).to_string()
//    }
//}

/// A connection point on a `SchematicSymbol`.
#[derive(Debug, PartialEq, Clone, Default)]
#[non_exhaustive]
pub struct SymbolConnection {
    /// Distance from top left of image horizontally in percentage of full width.
    pub x: u64,
    /// Distance from top left of image vertically in percentage of full height.
    pub y: u64,
    /// Which directions wires/cables are allowed to connect to this connection.
    pub allowed_connection_directions: ConnectionDirection,
}

bitflags! {
    /// `ConnectionDirection` specifies which direction(s) wires/cables are allowed to enter a
    /// connector.
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    pub struct ConnectionDirection: u8 {
        /// If connections are allowed from the top.
        const TOP = 0b0000_0001;
        /// If connections are allowed from the bottom.
        const BOTTOM = 0b0000_0010;
        /// If connections are allowed from the right.
        const RIGHT = 0b0000_0100;
        /// If connections are allowed from the left.
        const LEFT = 0b0000_1000;
}
}
