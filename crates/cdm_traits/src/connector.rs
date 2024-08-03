/// `Connector` contains common methods for various specific connector types defined in the library
pub trait Connector {
    /// `pin_count` returns the total number of pins of a connector
    fn pin_count(&self) -> u64;

    //TODO: add more methods here
}
