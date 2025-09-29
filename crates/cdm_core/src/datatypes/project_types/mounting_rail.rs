use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::datatypes::{
    internal_types::physical_location::PhysicalLocation,
    unit_helper::Length,
    util_types::{IECCodes, UserFields},
};

/// `MountingRail` represents an individual mounting rail in a project
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MountingRail {
    pub mounting_rail_type: String,
    pub length: Length,
    /// physical location of MountingRail
    pub physical_location: Option<PhysicalLocation>,
    /// Fields for use with IEC project coding
    pub iec_codes: Option<IECCodes>,
    /// User defined fields
    pub user_fields: Option<UserFields>,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}
