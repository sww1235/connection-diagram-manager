use std::path::PathBuf;

use crate::{datatypes::library_types::Library, error::Error};

/// `Connector` contains common methods for various specific connector types defined in the library
pub trait Connector {
    /// `pin_count` returns the total number of pins of a connector
    ///
    /// # Errors
    ///
    /// Will error if the connector type is not found in the provided library
    fn pin_count(&self, library: &Library) -> Result<u64, Error>;

    //TODO: add more methods here
}

/// `FromFile` indicates a data type was read in from a file, and contains the name of the file it
/// was read in from.
pub trait FromFile {
    /// `datafile` returns the datafile path which this instance was read in from
    fn datafile(&self) -> PathBuf;
}
