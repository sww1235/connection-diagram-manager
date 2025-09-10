use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::datatypes::internal_types::svg::Svg;

/// `SchematicSymbolType` represents a schematic symbol type
/// used in schematics to represent components
/// in schematic diagrams.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
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
