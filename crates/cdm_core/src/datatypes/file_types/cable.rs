use serde::{Deserialize, Serialize};

use crate::datatypes::{
    unit_helper::length::Length,
    util_types::{IECCodes, PhysicalLocation, UserFields},
};

/// `Cable` is the source code representation of the on-disk file format for an in-memory
/// [`Cable`](crate::datatypes::project_types::cable::Cable).
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Cable {
    /// The `CableType` key of this instance.
    pub cable_type: String,
    /// The structured name of the `Cable` instance.
    pub identifier: String,
    /// Optional description.
    pub description: Option<String>,
    /// length of wire or cable.
    pub length: Length,
    /// Pathway key containing instance.
    pub pathway: Option<String>,
    /// physical location of Cable.
    pub physical_location: Option<PhysicalLocation>,
    /// Fields for use with IEC project coding.
    pub iec_codes: Option<IECCodes>,
    /// User defined fields.
    pub user_fields: Option<UserFields>,
}
