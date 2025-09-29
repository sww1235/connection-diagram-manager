use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::datatypes::{
    unit_helper::Length,
    util_types::{IECCodes, PhysicalLocation, UserFields},
};

/// `Pathway` represents a physical instance of a pathway
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Pathway {
    /// Type of pathway
    pub path_type: String,
    /// structured identifier of pathway
    pub identifier: Option<String>,
    /// Optional description
    pub description: Option<String>,
    /// length
    pub length: Length,
    /// physical location of Pathway
    pub physical_location: Option<PhysicalLocation>,
    /// Fields for use with IEC project coding
    pub iec_codes: Option<IECCodes>,
    /// User defined fields
    pub user_fields: Option<UserFields>,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}
