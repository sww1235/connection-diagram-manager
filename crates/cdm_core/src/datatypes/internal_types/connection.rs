use std::cell::RefCell;
use std::rc::Rc;

use crate::datatypes::internal_types::{
    cable, equipment::Equipment, equipment_type::EquipConnector, location::Location,
    term_cable_type, wire,
};

// WireCableConnector can connect to WireCableConnector, wire_cable

/// `Connection` represents a connection between two different elements. Use methods to create as
/// they perform data validation.
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Connection {
    /// `wire_connector` is a connector on a `Wire`
    wire_connectors: Option<Vec<wire::Connector>>,
    /// `cable_connector` is a connector on a `Cable`
    cable_connectors: Option<Vec<cable::Connector>>,
    /// `term_cable_connector` is a connector on a `TermCable`
    term_cable_connectors: Option<Vec<term_cable_type::Connector>>,
    /// `equipment` is a piece of equipment
    equipment: Option<Rc<RefCell<Equipment>>>,
    /// `equipment_connector` is a reference to a connector on the specified `equipment`
    equipment_connector: Option<EquipConnector>,
    /// where the `connection` is located
    location: Rc<RefCell<Location>>,
}

impl Connection {
    /// Creates a new connection
    ///
    /// # Errors
    ///
    /// Will error if too many or too few connection parameters are passed in
    ///
    #[allow(clippy::unwrap_in_result, clippy::missing_panics_doc)]
    pub fn new(
        wire_connectors: Option<Vec<wire::Connector>>,
        cable_connectors: Option<Vec<cable::Connector>>,
        term_cable_connectors: Option<Vec<term_cable_type::Connector>>,
        equipment: Option<Rc<RefCell<Equipment>>>,
        equipment_connector: Option<EquipConnector>,
        location: Rc<RefCell<Location>>,
    ) -> Result<Self, Error> {
        if wire_connectors.is_some()
            && cable_connectors.is_some()
            && term_cable_connectors.is_some()
            && (equipment.is_some() || equipment_connector.is_some())
        {
            return Err(Error::TooManyArguments("A connection may only be defined between two of WireConnector,CableConnector, TermCableConnector, or EquipConnector. Reduce the number of Some() arguments.".to_owned()));
        }
        if (equipment.is_some() && equipment_connector.is_none())
            || (equipment.is_none() && equipment_connector.is_some())
        {
            return Err(Error::MismatchedArguments(
                "equipment and equipment_connector must both be defined together".to_owned(),
            ));
        }

        // if number of connectors specified is more than 2
        // allowing all these, since unwrap is checked beforehand, and the unwrap_in_result is a
        // false positive
        #[allow(clippy::unwrap_used, clippy::arithmetic_side_effects)]
        if wire_connectors.is_some()
            && cable_connectors.is_some()
            && term_cable_connectors.is_some()
            && (wire_connectors.as_ref().unwrap().len()
                + cable_connectors.as_ref().unwrap().len()
                + term_cable_connectors.as_ref().unwrap().len())
                > 2
        {
            return Err(Error::TooManyArguments("More than two WireConnectors, CableConnectors and TermCableConnectors are specified. A connection may only be defined between 2 connectors".to_owned()));
        }

        // if number of connectors is less than 2
        // allowing all these, since unwrap is checked beforehand, and the unwrap_in_result is a
        // false positive
        #[allow(clippy::unwrap_used, clippy::arithmetic_side_effects)]
        if wire_connectors.is_some()
            && cable_connectors.is_some()
            && term_cable_connectors.is_some()
            && (wire_connectors.as_ref().unwrap().len()
                + cable_connectors.as_ref().unwrap().len()
                + term_cable_connectors.as_ref().unwrap().len())
                < 2
        {
            return Err(Error::NotEnoughArguments("Less than two WireCableConnectors and TermCableConnectors are specified. A connection must have 2 connectors.".to_owned()));
        }

        // TODO: validate that number of pins is the same on both connectors

        Ok(Self {
            wire_connectors,
            cable_connectors,
            term_cable_connectors,
            equipment,
            equipment_connector,
            location,
        })
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
