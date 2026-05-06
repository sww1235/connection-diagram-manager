use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::datatypes::{
    project_types::enclosure::MountPoint,
    util_types::{IECCodes, PhysicalLocation, UserFields},
};

/// `Enclosure` is the source code representation of the on-disk file format for an in-memory
/// [`Enclosure`](crate::datatypes::project_types::enclosure::Enclosure).
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Enclosure {
    /// Type of location.
    pub enclosure_type: String,
    /// structured identifier of location.
    pub identifier: Option<String>,
    /// Optional description.
    pub description: Option<String>,
    /// physical location of Pathway.
    pub physical_location: Option<PhysicalLocation>,
    /// Fields for use with IEC project coding.
    pub iec_codes: Option<IECCodes>,
    /// User defined fields.
    pub user_fields: Option<UserFields>,
    /// `mount_point` - Actual locations of associated equipment within location.
    pub mount_points: BTreeMap<String, MountPoint>,
}
