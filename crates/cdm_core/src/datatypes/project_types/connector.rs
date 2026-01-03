use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    datatypes::{library_types::Library, util_types::SymbolStyle},
    error::LibraryError,
    traits::{self, FromFile, ProjectData},
};

/// `Connector` is an instance of a [`ConnectorType`](super::connector_type::ConnectorType)
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[expect(clippy::partial_pub_fields, reason = "contained_datafile_path is not part of public API")]
pub struct Connector {
    /// The type of this connector instance
    pub connector_type: String,
    /// Optional styling data for schematic symbol
    pub symbol_style: Option<SymbolStyle>,
    /// datafile the struct instance was read in from
    #[serde(skip)]
    pub(crate) contained_datafile_path: PathBuf,
}

impl traits::Connector for Connector {
    #[expect(
        clippy::unwrap_in_result,
        reason = "if somehow this library is used on a 128 bit architecture, I want a panic so people bug me and I can \
                  rearchitect the library to accomodate"
    )]
    #[inline(never)]
    fn pin_count(&self, library: &Library) -> Result<u64, LibraryError> {
        let connector_type = library
            .connector_types
            .get(&self.connector_type)
            .ok_or(LibraryError::ValueNotFound {
                id: self.connector_type.clone(),
                library_type: "Connector Type".to_owned(),
            })?;

        #[expect(
            clippy::unwrap_used,
            reason = "if somehow this library is used on a 128 bit architecture, I want a panic so people bug me and I can \
                      rearchitect the library to accomodate"
        )]
        Ok(u64::try_from(connector_type.pins.len()).unwrap())
    }
}
impl FromFile for Connector {
    #[inline]
    fn datafile(&self) -> PathBuf {
        self.contained_datafile_path.clone()
    }
    #[inline]
    fn set_datafile(&mut self, datafile_path: &Path) {
        self.contained_datafile_path = datafile_path.to_path_buf();
    }
}
impl ProjectData for Connector {}
