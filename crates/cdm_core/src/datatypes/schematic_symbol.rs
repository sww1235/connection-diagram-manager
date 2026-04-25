use std::{
    collections::{BTreeMap, HashSet},
    fmt,
};

use egui::{
    Pos2,
    Sense,
    Ui,
    Vec2,
    epaint::emath::GuiRounding as _,
    response::Response,
    widgets::{Image, ImageSource, Widget},
};
use log::trace;

use crate::{
    datatypes::{Library, Project, project_types::ProjectData, svg::Svg},
    error::Error,
};

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
    /// Map of connection points on symbol. The ID must be unique within the symbol.
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
        //trace!("scale: {}", self.scale);

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
            Vec2::new(original_width, original_height).round_ui()
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
        (self.original_symbol_dimensions() * self.scale).round_ui()
    }
}

/// A connection point on a `SchematicSymbol`.
#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
pub struct SymbolConnection {
    /// Distance from top left of image horizontally in percentage of full width.
    pub x: f32,
    /// Distance from top left of image vertically in percentage of full height.
    pub y: f32,
    /// Which directions wires/cables are allowed to connect to this connection.
    pub allowed_connection_directions: HashSet<ConnectionDirection>,
}

impl Default for SymbolConnection {
    #[inline]
    fn default() -> Self {
        let mut allowed_connection_directions = HashSet::from([ConnectionDirection::NONE]);
        allowed_connection_directions.shrink_to(5);
        Self {
            x: f32::default(),
            y: f32::default(),
            allowed_connection_directions,
        }
    }
}

/// `ConnectionDirection` specifies which direction(s) wires/cables are allowed to enter a
/// connector. Boolean parameter indicates if angled connections are allowed, or just a
/// perpendicular connection.
#[expect(clippy::exhaustive_enums, reason = "Only orthoganal connections allowed at the moment")]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum ConnectionDirection {
    /// If connections are allowed from the top.
    TOP,
    /// If connections are allowed from the bottom.
    BOTTOM,
    /// If connections are allowed from the right.
    RIGHT,
    /// If connections are allowed from the left.
    LEFT,
    /// If no connections are allowed.
    #[default]
    NONE,
}

//TODO: add in combo directions
impl ConnectionDirection {
    /// Returns a string representation of the direction.
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            ConnectionDirection::TOP => "Top",
            ConnectionDirection::BOTTOM => "Bottom",
            ConnectionDirection::RIGHT => "Right",
            ConnectionDirection::LEFT => "Left",
            ConnectionDirection::NONE => "None",
        }
    }

    /// All `ConnectionDirection`s.
    #[must_use]
    #[inline]
    pub fn all() -> HashSet<ConnectionDirection> {
        HashSet::from([
            ConnectionDirection::TOP,
            ConnectionDirection::BOTTOM,
            ConnectionDirection::RIGHT,
            ConnectionDirection::LEFT,
        ])
    }

    /// Horizontal `ConnectionDirection`s.
    #[must_use]
    #[inline]
    pub fn horizontal() -> HashSet<ConnectionDirection> {
        HashSet::from([ConnectionDirection::RIGHT, ConnectionDirection::LEFT])
    }

    /// Vertical `ConnectionDirection`s.
    #[must_use]
    #[inline]
    pub fn vertical() -> HashSet<ConnectionDirection> {
        HashSet::from([ConnectionDirection::TOP, ConnectionDirection::BOTTOM])
    }
}

impl fmt::Display for ConnectionDirection {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// `SchematicRepresentation` provides a SVG symbol used for drawing schematic diagrams.
pub trait SchematicRepresentation
where Self: ProjectData
{
    /// Returns a copy of the `SchematicSymbol` of the entity.
    ///
    /// If `schematic_symbol` is `None` then this shall return a placeholder warning graphic
    /// instead. This placeholder graphic is `Svg::default()`.
    fn schematic_symbol(&self) -> SchematicSymbol;

    /// Returns a mutable reference to the `SchematicSymbol` of the entitity.
    fn schematic_symbol_mut(&mut self) -> &mut SchematicSymbol;

    /// Update the scale parameter in the symbol.
    fn update_symbol_scale(&mut self, scale: f32);

    /// Set the symbol position in the GUI.
    fn set_symbol_position(&mut self, position: Pos2);

    /// Returns the current symbol position.
    fn symbol_position(&self) -> Pos2;

    /// Updates the `schematic_symbol` in `Self` from the options defined in `&self.entity_type`.
    ///
    /// `symbol_selector` selects an alternate symbol. If the variable is
    /// `None` or larger than `vec.len()-1`, the recommended implementation is to return the SVG at
    /// index `0`.
    ///
    /// # Errors
    ///
    /// Shall error if the id of `&self.entity_type` is not found in the provided library or other
    /// implementation specific errors.
    #[expect(clippy::result_large_err, reason = "Using main Error type")]
    fn update_schematic_symbol_from_library(
        &mut self,
        library: &Library,
        symbol_selector: Option<usize>,
        entity_id: String,
    ) -> Result<(), Error>;

    /// Updates tagged attributes within the `schematic_symbol` defined on `&self` based on data from `&self`
    /// or its library type.
    ///
    /// Also updates data within `self` during SVG parsing from the `library` or `project`.
    ///
    /// Can be called multiple times to update data if it changes.
    ///
    /// # Errors
    ///
    /// XML parsing or writing may fail.
    #[expect(clippy::result_large_err, reason = "Using main Error type")]
    fn update_symbol_data(&mut self, library: &Library, project: &Project) -> Result<(), Error>;
}
