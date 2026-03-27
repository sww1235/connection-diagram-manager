use std::collections::BTreeMap;

use bitflags::bitflags;
use egui::{
    Pos2,
    Sense,
    Ui,
    Vec2,
    response::Response,
    widgets::{Image, ImageSource, Widget},
};
use log::trace;

use super::svg::Svg;

/// `SchematicSymbol` represents an instance of a `SchematicSymbolType`.
///
/// It contains data and information about a specific symbol in the schematic.
///
/// It is used to hold data for specific instances of symbols that may be updated during the use of
/// the application.
#[derive(Debug, PartialEq, Clone, Default)]
#[non_exhaustive]
#[expect(clippy::partial_pub_fields, reason = "some fields are not part of the public API")]
pub struct SchematicSymbol {
    /// ID of `SymbolType` in library.
    pub symbol_type: String,
    /// Raw SVG data.
    pub(crate) visual_representation: Svg,
    // This is updated during creation in update_schematic_symbols_from_library()
    /// Identifier of symbol.
    pub identifier: String,
    /// Map of connection points on symbol. The ID must be unique per symbol.
    pub connections: BTreeMap<String, SymbolConnection>,
    /// Position of `SchematicSymbol` on screen.
    pub position: Pos2,
    /// How much to scale the symbol. A value of 1 is the dimensions specified in the original SVG.
    /// Negative values make the image smaller, positive numbers make the image bigger.
    ///
    /// Due to the use of `f32` datatype, rounding may occur.
    pub scale: f32,
}

impl Widget for &mut SchematicSymbol {
    #[inline]
    fn ui(self, ui: &mut Ui) -> Response {
        let sense_settings = Sense::click_and_drag();
        let uri = self.uri();

        let svg_data = self.visual_representation.get_data().into_bytes();
        //TODO: set sensible max_height() and max_width() here (maybe the size of the
        //window rectangle or something?
        //
        //TODO: revisit scaling here
        let image = Image::new(ImageSource::Bytes {
            uri: uri.into(),
            bytes: svg_data.into(),
        })
        .sense(sense_settings)
        .fit_to_original_size(self.scale);
        trace!("scale: {}", self.scale);

        ui.add(image)
    }
}

impl SchematicSymbol {
    /// Returns a standard URI for the SVG data.
    #[must_use]
    #[inline]
    pub fn uri(&self) -> String {
        format!("bytes://{}_schematic_symbol.svg", self.identifier).to_string()
    }

    /// Returns the original dimensions of the symbol.
    #[must_use]
    #[inline]
    pub fn original_symbol_dimensions(&self) -> Vec2 {
        if let Some(original_width) = self.visual_representation.get_original_width()
            && let Some(original_height) = self.visual_representation.get_original_height()
        {
            Vec2::new(original_width, original_height)
        } else {
            //TODO: maybe return option instead?
            Vec2::NAN
        }
    }

    /// Returns the size of the symbol scaled by the internal symbol scale parameter.
    #[must_use]
    #[inline]
    #[expect(clippy::arithmetic_side_effects, reason = "/shrug")]
    pub fn scaled_size(&self) -> Vec2 {
        self.original_symbol_dimensions() * self.scale
    }
}

/// A connection point on a `SchematicSymbol`.
#[derive(Debug, PartialEq, Clone, Default)]
#[non_exhaustive]
pub struct SymbolConnection {
    /// Distance from top left of image horizontally in percentage of full width.
    pub x: f32,
    /// Distance from top left of image vertically in percentage of full height.
    pub y: f32,
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
