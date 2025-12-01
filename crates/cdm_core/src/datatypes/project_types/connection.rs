use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// `Connection` represents a connection between two different elements.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[expect(clippy::partial_pub_fields, reason = "contained_datafile_path is not part of public API")]
pub struct Connection {
    /// ID of one end of connection
    pub end1: ConnectionType,
    /// ID of other end of connection
    pub end2: ConnectionType,
    /// datafile the struct instance was read in from
    #[serde(skip)]
    pub(crate) contained_datafile_path: PathBuf,
}

/// `ConnectionType` is an enum that defines what type of things can be connected to what other
/// type of things
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ConnectionType {
    /// An ID of a [`Wire`]
    Wire(String),
    /// An ID of a [`Cable`]
    Cable(String),
    /// An ID of a [`TermCable`]
    TermCable(String),
    /// An ID of an [`Equipment`]
    Equipment(String),
    /// An ID of a [`TerminalStrip`]
    TerminalStrip(String),
    /// An ID of a [`Connector`]
    Connector(String),
}
