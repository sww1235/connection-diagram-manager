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
