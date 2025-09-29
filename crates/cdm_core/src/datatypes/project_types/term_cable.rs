use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{
    datatypes::{
        library_types::Library,
        unit_helper::Length,
        util_types::{IECCodes, PhysicalLocation, UserFields},
    },
    error::Error,
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
    pub fn len(&self, library: &Library) -> Result<Length, Error> {
        let term_cable_type = library
            .term_cable_types
            .get(&self.term_cable_type)
            .ok_or(Error::LibraryValueNotFound(self.term_cable_type.clone()))?;

        Ok(term_cable_type
            .actual_length
            .clone()
            .unwrap_or(term_cable_type.nominal_length.clone().unwrap_or_default()))
    }
}
