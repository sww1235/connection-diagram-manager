use serde::{Deserialize, Serialize};

use crate::datatypes::{
    unit_helper::length::Length,
    util_types::{IECCodes, PhysicalLocation, UserFields},
};

/// `Pathway` is the source code representation of the on-disk file format for an in-memory
/// [`Pathway`](crate::datatypes::project_types::pathway::Pathway).
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Pathway {
    /// Type of pathway.
    pub path_type: String,
    /// structured identifier of pathway.
    pub identifier: Option<String>,
    /// Optional description.
    pub description: Option<String>,
    /// length.
    pub length: Length,
    /// physical location of Pathway.
    pub physical_location: Option<PhysicalLocation>,
    /// Fields for use with IEC project coding.
    pub iec_codes: Option<IECCodes>,
    /// User defined fields.
    pub user_fields: Option<UserFields>,
}
