use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    datatypes::{
        unit_helper::length::Length,
        util_types::{IECCodes, PhysicalLocation, UserFields},
    },
    traits::FromFile,
};

/// `Pathway` represents a physical instance of a pathway
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[expect(clippy::partial_pub_fields)]
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
    #[serde(skip)]
    pub(crate) contained_datafile_path: PathBuf,
}

impl FromFile for Pathway {
    fn datafile(&self) -> PathBuf {
        self.contained_datafile_path.clone()
    }
    fn set_datafile(&mut self, datafile_path: &Path) {
        self.contained_datafile_path = datafile_path.to_path_buf();
    }
}
