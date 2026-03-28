use std::path::{Path, PathBuf};

use egui::Pos2;

use crate::{
    datatypes::{library_types::Library, project_types::Project, schematic_symbol::SchematicSymbol, svg::Svg},
    error::{Error, LibraryError},
};

/// `Connector` contains common methods for various specific connector types defined in the
/// library.
pub trait Connector {
    /// `pin_count` returns the total number of pins of a connector.
    ///
    /// # Errors
    ///
    /// Will error if the connector type is not found in the provided library.
    fn pin_count(&self, library: &Library) -> Result<u64, LibraryError>;

    //TODO: add more methods here
}

/// `FromFile` indicates a data type was read in from a file and can be read in from a file, and
/// contains the name of the file it was read in from.
pub trait FromFile {
    /// `datafile` returns the datafile path which this instance was read in from.
    fn datafile(&self) -> PathBuf;

    /// `set_file` sets the datafile path in the struct.
    fn set_datafile(&mut self, datafile_path: &Path);
}

/// `VisualRepresentation` provides a SVG image of the entity for use in rendering.
pub trait VisualRepresentation {
    /// returns a standard or representative representation of the entity in SVG format.
    fn visual_rep(&self, library: &Library) -> Svg;
    //TODO: add an update_data function here
}
/// `SchematicRepresentation` provides a SVG symbol used for drawing schematic diagrams.
pub trait SchematicRepresentation
where Self: ProjectData
{
    /// Returns the SVG `schematic_symbol` of the entity and a URI used in rendering code.
    ///
    /// If `schematic_symbol` is `None` then this shall return a placeholder warning graphic
    /// instead.
    fn schematic_symbol(&self) -> SchematicSymbol;

    /// Update the scale parameter in the symbol.
    fn update_symbol_scale(&mut self, scale: f32);

    /// Set the symbol position in the GUI.
    fn set_symbol_position(&mut self, position: Pos2);

    //TODO: somehow make URI an element of SVG rather than being built in the trait method.
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
/// Marker trait for Project data.
pub trait ProjectData {}

/// Marker trait for Library data.
pub trait LibraryData {}
