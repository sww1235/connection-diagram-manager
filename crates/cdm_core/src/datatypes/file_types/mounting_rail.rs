use serde::{Deserialize, Serialize};

use crate::datatypes::{
    unit_helper::length::Length,
    util_types::{IECCodes, PhysicalLocation, UserFields},
};

/// `MountingRail` is the source code representation of the on-disk file format for an in-memory
/// [`MountingRail`](crate::datatypes::project_types::mounting_rail::MountingRail).
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MountingRail {
    /// ID of type of mounting rail.
    pub mounting_rail_type: String,
    /// The structured name of the `MountingRail` instance.
    pub identifier: String,
    /// Optional description.
    pub description: Option<String>,
    /// Length of mounting rail.
    pub length: Length,
    /// physical location of `MountingRail`.
    pub physical_location: Option<PhysicalLocation>,
    /// Fields for use with IEC project coding.
    pub iec_codes: Option<IECCodes>,
    /// User defined fields.
    pub user_fields: Option<UserFields>,
}
