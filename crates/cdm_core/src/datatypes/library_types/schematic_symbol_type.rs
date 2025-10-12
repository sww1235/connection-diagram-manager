use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{datatypes::svg::Svg, traits::FromFile};

/// `SchematicSymbolType` represents a schematic symbol type
/// used in schematics to represent components
/// in schematic diagrams.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SchematicSymbolType {
    /// Short name for display. Can contain spaces/special characters
    pub name: String,
    /// What to actually display when symbol is rendered on drawing
    pub visual_representation: Svg,
    /// Optional description
    pub description: Option<String>,
    /// If this symbol supports links. If true, schematic symbol instances
    /// will be allowed to define links between parent and child components
    pub supports_links: bool,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}
impl FromFile for SchematicSymbolType {
    fn datafile(&self) -> PathBuf {
        self.contained_datafile_path.clone()
    }
}
