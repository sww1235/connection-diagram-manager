use std::cell::RefCell;
use std::rc::Rc;

use crate::datatypes::internal_types::{connector::Connector, location::Location};

/// `Connection` represents a connection between two different elements. Use methods to create as
/// they perform data validation.
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Connection {
    /// connectors
    connectors: (Rc<RefCell<Connector>>, Rc<RefCell<Connector>>),
    /// where the `connection` is located
    location: Rc<RefCell<Location>>,
}

impl Connection {
    /// Creates a new connection
    #[must_use]
    pub fn new(
        connector1: Rc<RefCell<Connector>>,
        connector2: Rc<RefCell<Connector>>,
        location: Rc<RefCell<Location>>,
    ) -> Self {
        // TODO: validate that number of pins is the same on both connectors

        Self {
            connectors: (connector1, connector2),
            location,
        }
    }

    /// `location` returns a shared reference to the location of the `Connection`
    #[must_use]
    pub fn location(&self) -> Rc<RefCell<Location>> {
        Rc::clone(&self.location)
    }
}

#[non_exhaustive]
#[derive(Debug)]
/// `Error` is the error returned by methods operating on `Connection` objects
pub enum Error {
    /// `TooManyArguments` means a `Connection` object has too many arguments
    TooManyArguments(String),
    /// `NotEnoughArguments` means a `Connection` object has too few arguments
    NotEnoughArguments(String),
    /// `MismatchedArguments` means a `Connection` object has mismatched arguments
    MismatchedArguments(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::TooManyArguments(ref contents)
            | Error::NotEnoughArguments(ref contents)
            | Error::MismatchedArguments(ref contents) => {
                write!(f, "{contents}")
            }
        }
    }
}
