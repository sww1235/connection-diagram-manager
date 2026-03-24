use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// `Connection` represents a connection between two different elements.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[expect(clippy::partial_pub_fields, reason = "contained_datafile_path is not part of public API")]
pub struct Connection {
    /// ID of one end of connection.
    pub end1: Type,
    /// ID of other end of connection.
    pub end2: Type,
    /// datafile the struct instance was read in from.
    #[serde(skip)]
    pub(crate) contained_datafile_path: PathBuf,
}

/// `Type` is an enum that defines what type of things can be connected to what other
/// type of things.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Type {
    /// A [`Wire`].
    Wire {
        /// ID of [`Wire`] instance in [`Project`] hashmap.
        wire_id: String,
    },
    //TODO: which cable core?
    /// A [`Cable`].
    Cable {
        /// ID of [`Cable`] instance in [`Project`] hashmap.
        cable_id: String,
    },
    //TODO: which cable core? Or should this be connector based?
    /// A [`TermCable`].
    TermCable {
        /// ID of [`TermCable`] instance in [`Project`] hashmap.
        cable_id: String,
    },
    /// An [`Equipment`].
    Equipment {
        /// ID of [`Equipment`] instance in [`Project`] hashmap.
        equipment_id: String,
        /// ID of connection point on equipment.
        connection_point_id: String,
    },
    /// A [`TerminalStrip`].
    TerminalStrip {
        /// ID of [`TerminalStrip`] in [`Project`] hashmap.
        term_strip_id: String,
        /// ID or index of element within terminal strip.
        element_id: String,
    },
    /// An [`Connector`].
    Connector {
        /// ID of [`Connector`] instance in [`Project`] hashmap.
        connector_id: String,
        /// ID or index of connector pin.
        pin_id: String,
    },
}
