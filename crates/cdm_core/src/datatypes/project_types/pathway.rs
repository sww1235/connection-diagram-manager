use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    datatypes::{
        file_types,
        project_types::ProjectData,
        unit_helper::length::Length,
        util_types::{IECCodes, PhysicalLocation, UserFields},
    },
    traits::FromFile,
};

/// `Pathway` represents a physical instance of a pathway.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[expect(clippy::partial_pub_fields, reason = "contained_datafile_path is not part of public API")]
pub struct Pathway {
    /// Type of pathway.
    pub pathway_type: String,
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
    /// datafile the struct instance was read in from.
    #[serde(skip)]
    pub(crate) contained_datafile_path: PathBuf,
}

impl From<file_types::pathway::Pathway> for Pathway {

    #[inline]
    fn from(value: file_types::pathway::Pathway) -> Self {
        Self{
            pathway_type: value.pathway_type,
            length: value.length,
            identifier: value.identifier,
            description: value.description,
            physical_location: value.physical_location,
            iec_codes: value.iec_codes,
            user_fields: value.user_fields,
            contained_datafile_path: PathBuf::new(),
        }
    }
}

impl FromFile for Pathway {
    #[inline]
    fn datafile(&self) -> PathBuf {
        self.contained_datafile_path.clone()
    }
    #[inline]
    fn set_datafile(&mut self, datafile_path: &Path) {
        self.contained_datafile_path = datafile_path.to_path_buf();
    }
}

impl ProjectData for Pathway {}
