use std::path::{Path, PathBuf};

use crate::{
    datatypes::{library_types::Library, svg::Svg},
    error::LibraryError,
};

/// `Connector` contains common methods for various specific connector types defined in the library
pub trait Connector {
    /// `pin_count` returns the total number of pins of a connector
    ///
    /// # Errors
    ///
    /// Will error if the connector type is not found in the provided library
    fn pin_count(&self, library: &Library) -> Result<u64, LibraryError>;

    //TODO: add more methods here
}

/// `FromFile` indicates a data type was read in from a file and can be read in from a file, and
/// contains the name of the file it was read in from.
pub trait FromFile {
    /// `datafile` returns the datafile path which this instance was read in from
    fn datafile(&self) -> PathBuf;

    /// `set_file` sets the datafile path in the struct
    fn set_datafile(&mut self, datafile_path: &Path);
}

/// `SchematicRepresentation` provides a SVG symbol used for drawing schematic diagrams
pub trait SchematicRepresentation {
    /// returns a SVG schematic symbol of the entity.
    ///
    /// `symbol_selector` selects an alternate symbol for a specific entity. If the variable is
    /// `None` or larger than `vec.len()-1`, the recommended implementation is to return the SVG at
    /// index `0`.
    ///
    /// # Errors
    ///
    /// Will error if the id of `&self.entity_type` is not found in the provided library or other
    /// implementation specific errors
    fn schematic_symbol(&self, library: &Library, symbol_selector: Option<usize>) -> Result<Svg, LibraryError>;
}
/// Marker trait for Project data
pub trait ProjectData {}

/// Marker trait for Library data
pub trait LibraryData {}
