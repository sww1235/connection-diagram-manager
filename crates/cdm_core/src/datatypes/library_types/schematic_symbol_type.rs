use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    datatypes::svg::Svg,
    traits::{FromFile, LibraryData},
};

/// `SchematicSymbolType` represents a schematic symbol type
/// used in schematics to represent components
/// in schematic diagrams.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[expect(clippy::partial_pub_fields, reason = "contained_datafile_path is not part of public API")]
pub struct SchematicSymbolType {
    /// Short name for display. Can contain spaces/special characters.
    pub name: String,
    /// What to actually display when symbol is rendered on drawing.
    pub visual_representation: Svg,
    /// Optional description.
    pub description: Option<String>,
    /// If this symbol supports links. If true, schematic symbol instances
    /// will be allowed to define links between parent and child components.
    pub supports_links: bool,
    /// datafile the struct instance was read in from.
    #[serde(skip)]
    pub(crate) contained_datafile_path: PathBuf,
}
impl FromFile for SchematicSymbolType {
    #[inline]
    fn datafile(&self) -> PathBuf {
        self.contained_datafile_path.clone()
    }
    #[inline]
    fn set_datafile(&mut self, datafile_path: &Path) {
        self.contained_datafile_path = datafile_path.to_path_buf();
    }
}

impl LibraryData for SchematicSymbolType {}
