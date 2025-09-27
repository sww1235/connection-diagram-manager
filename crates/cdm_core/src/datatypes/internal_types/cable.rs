use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::datatypes::{
    internal_types::physical_location::PhysicalLocation,
    unit_helper::Length,
    util_types::{IECCodes, UserFields},
};

/// `Cable` represents a particular instance of a `CableType`
/// It represents a physical item.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Cable {
    /// The `CableType` key of this instance
    pub cable_type: String,
    /// The structured name of the `Cable` instance
    pub identifier: Option<String>,
    /// Optional description
    pub description: Option<String>,
    /// length of wire or cable
    pub length: Length,
    /// Pathway key containing instance
    pub pathway: Option<String>,
    /// physical location of Cable
    pub physical_location: Option<PhysicalLocation>,
    /// Fields for use with IEC project coding
    pub iec_codes: Option<IECCodes>,
    /// User defined fields
    pub user_fields: Option<UserFields>,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}
