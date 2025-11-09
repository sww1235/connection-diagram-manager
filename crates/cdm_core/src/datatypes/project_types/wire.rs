use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{
    datatypes::{
        unit_helper::Length,
        util_types::{IECCodes, PhysicalLocation, UserFields},
    },
    traits::FromFile,
};

/// `Wire` represents a particular instance of a `WireType`.
/// It represents a physical item.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Wire {
    /// The `WireType` of this instance
    pub wire_type: String,
    /// The structured name of the `Wire` instance
    pub identifier: Option<String>,
    /// Optional description
    pub description: Option<String>,
    /// length of wire
    pub length: Length,
    /// physical location of Wire
    pub physical_location: Option<PhysicalLocation>,
    /// Fields for use with IEC project coding
    pub iec_codes: Option<IECCodes>,
    /// User defined fields
    pub user_fields: Option<UserFields>,
    /// Pathway containing instance
    pub pathway: Option<String>,
    /// One end of `Wire` / Cable.
    pub end1_connector_type: String,
    /// The other end of `Wire`
    pub end2_connector_type: String,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}

impl FromFile for Wire {
    fn datafile(&self) -> PathBuf {
        self.contained_datafile_path.clone()
    }
}
