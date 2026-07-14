use serde::{Deserialize, Serialize};

use crate::datatypes::util_types::{IECCodes, PhysicalLocation, UserFields};

/// `TermCable` is the source code representation of the on-disk file format for an in-memory
/// [`TermCable`](crate::datatypes::project_types::term_cable::TermCable).
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TermCable {
    /// The `TermCableType` of this instance.
    pub term_cable_type: String,
    /// The structured name of the `TermCable` instance.
    pub identifier: String,
    /// Optional description.
    pub description: Option<String>,
    /// Pathway containing instance.
    pub pathway: Option<String>,
    /// physical location of `TermCable`.
    pub physical_location: Option<PhysicalLocation>,
    /// Fields for use with IEC project coding.
    pub iec_codes: Option<IECCodes>,
    /// User defined fields.
    pub user_fields: Option<UserFields>,
}
