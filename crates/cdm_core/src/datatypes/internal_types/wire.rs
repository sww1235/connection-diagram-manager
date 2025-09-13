use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use cdm_macros::{Empty, Merge, PartialEmpty};
use cdm_traits::{connector, partial_empty::PartialEmpty};

use crate::datatypes::{
    internal_types::{connector_type::ConnectorType, pathway::Pathway, wire_type::WireType},
    unit_helper::Length,
};

/// `Wire` represents a particular instance of a `WireType`.
/// It represents a physical item.
#[derive(Debug, Default, PartialEq, Clone, Merge, PartialEmpty, Empty)]
pub struct Wire {
    /// Internal `id` of `Wire` instance
    pub id: String,
    /// The `WireType` of this instance
    pub wire_type: Rc<RefCell<WireType>>,
    /// The structured name of the `Wire` instance
    pub identifier: Option<String>,
    /// Optional description
    pub description: Option<String>,
    /// length of wire
    pub length: Length,
    /// Pathway containing instance
    pub pathway: Option<Rc<RefCell<Pathway>>>,
    /// One end of `Wire` / Cable.
    pub end1: Rc<RefCell<Connector>>,
    /// The other end of `Wire`
    pub end2: Rc<RefCell<Connector>>,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}

/// `Connector` is a connector on one end of a `Wire`
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Connector {
    /// `connector_type` represents the connector type that is on the end of a `Wire`
    pub connector_type: Rc<RefCell<ConnectorType>>,
}
impl Wire {
    /// Creates an empty instance of `WireCable`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl connector::Connector for Connector {
    fn pin_count(&self) -> u64 {
        #[expect(clippy::unwrap_used)]
        // allowing unwrap as I want a panic here if this application
        // is used on a 128 bit architecture
        u64::try_from(self.connector_type.borrow().pins.len()).unwrap()
    }
}
