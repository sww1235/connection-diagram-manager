use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::datatypes::{
    project_types::Project,
    unit_helper::Length,
    util_types::{IECCodes, PhysicalLocation, UserFields},
};

/// `TermCable` represents a particular instance of a `TermCableType`.
/// It represents a physical item.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TermCable {
    /// The `TermCableType` of this instance
    pub term_cable_type: String,
    /// The structured name of the `TermCable` instance
    pub identifier: Option<String>,
    /// Optional description
    pub description: Option<String>,
    /// Pathway containing instance
    pub pathway: Option<String>,
    /// physical location of TermCable
    pub physical_location: Option<PhysicalLocation>,
    /// Fields for use with IEC project coding
    pub iec_codes: Option<IECCodes>,
    /// User defined fields
    pub user_fields: Option<UserFields>,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}
impl TermCable {
    /// length of `TermCableType`
    #[must_use]
    pub fn len(&self, project: &Project) -> Length {
        self.term_cable_type.borrow().actual_length.unwrap_or(
            self.term_cable_type
                .borrow()
                .nominal_length
                .unwrap_or_default(),
        )
    }
}
