use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    datatypes::{
        library_types::Library,
        unit_helper::length::Length,
        util_types::{IECCodes, PhysicalLocation, UserFields},
    },
    error::LibraryError,
    traits::{FromFile, ProjectData},
};

/// `TermCable` represents a particular instance of a `TermCableType`.
/// It represents a physical item.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[expect(clippy::partial_pub_fields, reason = "contained_datafile_path is not part of public API")]
pub struct TermCable {
    /// The `TermCableType` of this instance.
    pub term_cable_type: String,
    /// The structured name of the `TermCable` instance.
    pub identifier: Option<String>,
    /// Optional description.
    pub description: Option<String>,
    /// Pathway containing instance.
    pub pathway: Option<String>,
    /// physical location of `TermCable`.
    pub physical_location: Option<PhysicalLocation>,
    /// Fields for use with IEC project coding.
    pub iec_codes: Option<IECCodes>,
    /// User defined fields.
    pub user_fields: Option<UserFields>,
    /// datafile the struct instance was read in from.
    #[serde(skip)]
    pub(crate) contained_datafile_path: PathBuf,
}
impl TermCable {
    /// length of `TermCableType`.
    ///
    /// # Errors
    ///
    /// Will error if `term_cable_type` id not found in provided library.
    #[inline(never)]
    pub fn len(&self, library: &Library) -> Result<Length, LibraryError> {
        let term_cable_type = library
            .term_cable_types
            .get(&self.term_cable_type)
            .ok_or(LibraryError::ValueNotFound {
                id: self.term_cable_type.clone(),
                //TODO: figure out how to insert the ID of the term cable here
                found_in: "term_cable".to_owned(),
                library_type: "Term Cable Type".to_owned(),
            })?;

        Ok(term_cable_type
            .actual_length
            .clone()
            .unwrap_or(term_cable_type.nominal_length.clone().unwrap_or_default()))
    }
}
impl FromFile for TermCable {
    #[inline]
    fn datafile(&self) -> PathBuf {
        self.contained_datafile_path.clone()
    }
    #[inline]
    fn set_datafile(&mut self, datafile_path: &Path) {
        self.contained_datafile_path = datafile_path.to_path_buf();
    }
}
impl ProjectData for TermCable {}
