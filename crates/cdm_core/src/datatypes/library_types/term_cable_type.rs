use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{
    datatypes::{
        library_types::{Library, LibraryData},
        project_types::connector::Connectorize,
        unit_helper::length::Length,
        util_types::{Catalog, LineStyle},
    },
    error::LibraryError,
    traits::FromFile,
};

/// `TermCableType` represents a terminated cable with 2 ends and a connector on at least 1 end.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[expect(clippy::partial_pub_fields, reason = "contained_datafile_path is not part of public API")]
pub struct TermCableType {
    /// Catalog information.
    pub catalog: Option<Catalog>,
    /// Underlying wire or cable type of Terminated Cable.
    pub wire_cable: WireCable,
    /// Nominal Length of Terminated Cable.
    pub nominal_length: Option<Length>,
    /// Actual Length of Terminated Cable.
    pub actual_length: Option<Length>,
    /// appearance in schematics.
    pub line_style: Option<LineStyle>,
    /// One end of Terminated Cable.
    pub end1: BTreeMap<String, Connector>,
    /// The other end of Terminated Cable.
    pub end2: BTreeMap<String, Connector>,
    /// datafile the struct instance was read in from.
    #[serde(skip)]
    pub(crate) contained_datafile_path: PathBuf,
}
impl FromFile for TermCableType {
    #[inline]
    fn datafile(&self) -> PathBuf {
        self.contained_datafile_path.clone()
    }
    #[inline]
    fn set_datafile(&mut self, datafile_path: &Path) {
        self.contained_datafile_path = datafile_path.to_path_buf();
    }
}

/// `WireCable` allows either a `WireType` or `CableType` to be the root of a `TermCableType`.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[expect(clippy::exhaustive_enums, reason = "Only these two options make sense in this enum")]
pub enum WireCable {
    /// `CableType`.
    CableType(String),
    /// `WireType`.
    WireType(String),
}

/// `TermCableConnectorTermination` represents the connections between a pin of an individual
/// `TermCableConnector` and the individual core of the cable.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Termination {
    /// `Core` represents which individual wire inside a cable this pin is connected to.
    pub core: String,
    /// `Pin` represents which pin in the associated connector the core is connected to.
    pub pin: String,
}

/// `Connector` represents a connector on one end of a `TermCable`.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Connector {
    /// `connector_type` represents the connector type that is on the end of a `TermCable`.
    pub connector_type: String,
    /// `terminations` represents the pin/core mapping for this connector.
    pub terminations: Vec<Termination>,
}

impl Connectorize for Connector {
    #[expect(clippy::unwrap_in_result, reason = "I want the panic on a 128bit architecture")]
    #[inline(never)]
    fn pin_count(&self, library: &Library) -> Result<u64, LibraryError> {
        let connector_type = library
            .connector_types
            .get(&self.connector_type)
            .ok_or(LibraryError::ValueNotFound {
                id: self.connector_type.clone(),
                //TODO: figure out how to insert the ID of the term cable connector here
                found_in: "term cable connector".to_owned(),
                library_type: "Connector Type".to_owned(),
            })?;
        #[expect(clippy::unwrap_used, reason = "I want the panic on a 128bit architecture")]
        // allowing unwrap as I want a panic here if this application
        // is used on a 128 bit architecture
        Ok(u64::try_from(connector_type.pins.len()).unwrap())
    }
}
impl LibraryData for TermCableType {}
